use anyhow;
use rand::random;
use tokio::task::JoinSet;
use std::cmp::{max, min};
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::select;
use tokio::sync::{broadcast, Mutex, RwLock};

use crate::binary::types::{Text, Vector2};
use crate::network::transpiled::item_slots;
use crate::network::messages::{
	AnglerQuest, ConnectionApprove, DropItem, KillCount, Message, NPCInfo,
	PillarShieldStrengths, SpawnResponse, WorldTotals, Sanitize, PlayerItemSlot,
};
use crate::network::client::{Client, ConnectionState, MAX_ITEM_SLOTS};
use crate::network::utils::{
	encode_world_header, get_section_x, get_section_y, get_sections_near,
};
use crate::world::types::World;

const GAME_VERSION: &str = "Terraria279";
const MAX_CLIENTS: usize = 256;
const MAX_NAME_LEN: usize = 20;
const TILE: f32 = 16.;

pub struct Server {
	pub world: RwLock<World>,
	pub password: RwLock<String>,
	pub clients: Mutex<[Option<Client>; MAX_CLIENTS]>,
	pub broadcast: broadcast::Sender<(Message, Option<usize>)>,
}

impl Server {
	pub fn new(world: World, password: &str) -> Server {
		// https://github.com/rust-lang/rust/issues/44796#issuecomment-967747810
		const INIT_CLIENT_NONE: Option<Client> = None;
		let (tx, _) = broadcast::channel(1024);

		Server {
			world: RwLock::new(world),
			password: RwLock::new(password.to_owned()),
			clients: Mutex::new([INIT_CLIENT_NONE; MAX_CLIENTS]),
			broadcast: tx,
		}
	}

	pub async fn listen(self, address: &str) -> io::Result<()> {
		let listener = TcpListener::bind(address).await?;
		let arc = Arc::new(self);

		loop {
			let (mut stream, addr) = listener.accept().await?;
			let rc = Arc::clone(&arc);
			let rc2 = Arc::clone(&arc);

			tokio::spawn(async move {
				let src = {
					let mut clients = rc.clients.lock().await;
					let Some(id) = clients.iter().position(Option::is_none) else {
						Message::ConnectionRefuse(Text::Key(
							"CLI.ServerIsFull".to_owned(),
							vec![],
						))
						.write_stream(Pin::new(&mut stream))
						.await
						.unwrap();
						return;
					};
					let world = rc.world.read().await;
					clients[id] = Some(Client::new(
						addr,
						get_section_x(world.header.width as usize) + 1,
						get_section_y(world.header.height as usize) + 1,
					));
					id
				};

				let _ = tokio::spawn(async move { rc.accept(&mut stream, src).await }).await;

				rc2.clients.lock().await[src] = None;
			});
		}
	}

	async fn accept(&self, stream: &mut TcpStream, src: usize) -> anyhow::Result<()> {
		let (mut rh, mut wh) = stream.split();
		let mut tx = self.broadcast.clone();
		let mut rx = self.broadcast.subscribe();

		loop {
			let mut length = [0u8; 2];
			select! {
				read_result = rh.read(&mut length) => {
					// Player disconnected
					if read_result.is_err() || read_result.is_ok_and(|l| l == 0) {
						self.clients.lock().await[src] = None;
						return Ok(());
					}

					let length = u16::from_le_bytes(length);
					if length < 2 {
						continue;
					}

					let mut buffer = vec![0u8; length as usize - 2];
					let read_result = rh.read_exact(&mut buffer).await;
					if read_result.is_err() || read_result.is_ok_and(|l| l == 0) {
						self.clients.lock().await[src] = None;
					} else {
						let response = self.handle_message(Message::from(buffer), src, &mut tx).await?;
						for msg in response {
							msg.write_stream(Pin::new(&mut wh)).await?;
						}
					}
				}
				content = rx.recv() => {
					let (content, ignore_id) = content?;
					if ignore_id.map_or(true, |id| id != src) {
						content.write_stream(Pin::new(&mut wh)).await?;
					}
				}
			}
		}
	}

	async fn handle_message(
		&self,
		msg: Message,
		src: usize,
		tx: &mut broadcast::Sender<(Message, Option<usize>)>,
	) -> anyhow::Result<Vec<Message>> {
		let mut clients = self.clients.lock().await;
		let client = clients[src].as_mut().unwrap();

		Ok(match msg {
			// The client sends their version and if it matches the server version, we send ConnectionApprove if there is not password and PasswordRequest if there is a password
			// If their version does not match, refuse connection
			Message::VersionIdentifier(version) => {
				if client.state != ConnectionState::New {
					return Ok(vec![]);
				}

				if version == GAME_VERSION {
					let password = self.password.read().await;
					if password.is_empty() {
						client.state = ConnectionState::Authenticated;
						vec![Message::ConnectionApprove(ConnectionApprove {
							client_id: src as u8,
							flag: false,
						})]
					} else {
						client.state = ConnectionState::PendingAuthentication;
						vec![Message::PasswordRequest]
					}
				} else {
					println!("Player tried joining with unsupported version {}", version);
					vec![Message::ConnectionRefuse(Text::Key(
						"LegacyMultiplayer.4".to_owned(),
						vec![],
					))]
				}
			}
			// The client sends back the password. If it's correct, we may send back their dedicated user id
			// Don't know what the "false" is but that's what the source code does
			// If the password is wrong, refuse connection
			Message::PasswordResponse(pass) => {
				let password = self.password.read().await;
				if pass == password.as_str() {
					client.state = ConnectionState::Authenticated;
					vec![Message::ConnectionApprove(ConnectionApprove {
						client_id: src as u8,
						flag: false,
					})]
				} else {
					vec![Message::ConnectionRefuse(Text::Key(
						"LegacyMultiplayer.1".to_owned(),
						vec![],
					))]
				}
			}
			// The player sends their character's UUID. Terraria seems to do nothing with it so let's just store it
			Message::UUID(uuid) => {
				client.uuid = Some(uuid);
				vec![]
			}
			// If another player already exists with the same name, refuse this player
			// The player sends character details upon first join. Store it
			// Broadcast this player to all other players
			Message::PlayerDetails(mut pd) => {
				if client.state != ConnectionState::Authenticated {
					return Ok(vec![Message::ConnectionRefuse(Text::Key(
						"LegacyMultiplayer.1".to_owned(),
						vec![],
					))]);
				}

				let exists_same_name = clients.iter().any(|c_opt| {
					c_opt.as_ref().map_or(false, |c| {
						c.details.as_ref().map_or(false, |d| d.name == pd.name)
					})
				});

				if exists_same_name {
					return Ok(vec![Message::ConnectionRefuse(Text::Key(
						"LegacyMultiplayer.5".to_owned(),
						vec![Text::Literal(pd.name)],
					))]);
				}

				if pd.name.len() > MAX_NAME_LEN {
					return Ok(vec![Message::ConnectionRefuse(Text::Key(
						"Net.NameTooLong".to_owned(),
						vec![],
					))]);
				}

				if pd.name.is_empty() {
					return Ok(vec![Message::ConnectionRefuse(Text::Key(
						"Net.EmptyName".to_owned(),
						vec![],
					))]);
				}

				// TODO: compare client difficulty with world difficulty

				pd.sanitize(src as u8);
				tx.send((Message::PlayerDetails(pd.clone()), Some(src)))?;
				let c = clients[src].as_mut().unwrap();
				c.details = Some(pd);
				c.state = ConnectionState::DetailsReceived;
				vec![]
			}
			Message::PlayerHealth(mut ph) => {
				ph.sanitize(src as u8);
				tx.send((Message::PlayerHealth(ph.clone()), Some(src)))?;
				client.health = Some(ph);
				vec![]
			}
			// Doesn't get broadcast
			Message::PlayerMana(mut pm) => {
				pm.sanitize(src as u8);
				client.mana = Some(pm);
				vec![]
			}
			Message::PlayerBuffs(mut pb) => {
				pb.sanitize(src as u8);
				tx.send((Message::PlayerBuffs(pb.clone()), Some(src)))?;
				client.buffs = Some(pb);
				vec![]
			}
			Message::PlayerLoadout(mut psl) => {
				psl.sanitize(src as u8);
				tx.send((Message::PlayerLoadout(psl.clone()), Some(src)))?;
				// TODO: maybe handle hide_accessory. still not sure if it matters to the server
				client.selected_loadout = psl.index;
				vec![]
			}
			Message::PlayerItemSlot(mut pis) => {
				pis.sanitize(src as u8);

				let loadout_start = item_slots::LOADOUTS_START[client.selected_loadout as usize];
				let loadout_end = item_slots::LOADOUTS_END[client.selected_loadout as usize];
				let idx = pis.slot_id as usize;

				// If the player sends their armor, copy it to their current loadout
				if (item_slots::ARMOR_START..=item_slots::DYES_END).contains(&idx) {
					client.items[idx - item_slots::ARMOR_START + loadout_start] = Some(pis.clone())
				}

				if idx < MAX_ITEM_SLOTS {
					tx.send((Message::PlayerItemSlot(pis.clone()), Some(src)))?;

					// Don't try to use the client's current loadout slots because it's always sent as 0
					if !(loadout_start..=loadout_end).contains(&idx) {
						client.items[idx] = Some(pis);
					}
				}
				vec![]
			}
			Message::WorldRequest => {
				// dbg!(client.items.iter().skip(60).take(4).collect::<Vec<&Option<PlayerItemSlot>>>());
				vec![encode_world_header(&self.world.read().await.header)]
				// todo: Main.SyncAnInvasion
			}
			Message::SpawnRequest(sr) => {
				if client.state != ConnectionState::DetailsReceived {
					return Ok(vec![Message::ConnectionRefuse(Text::Key(
						"LegacyMultiplayer.1".to_owned(),
						vec![],
					))]);
				}

				let w = self.world.read().await;
				let c = client;
				let mut res = vec![encode_world_header(&w.header)];
				let mut count = 0;

				// List<Point> portalSections;
				// PortalHelper.SyncPortalsOnPlayerJoin(this.whoAmI, 1, dontInclude, out portalSections);
				// sec_count += portalSections.Count;

				let x_max = get_section_x(w.header.width as usize) - 1;
				let y_max = get_section_y(w.header.height as usize) - 1;
				let (xs, xe, ys, ye) =
					get_sections_near(w.header.spawn_x, w.header.spawn_y, x_max, y_max);
				let mut secs = c.encode_sections(&w, xs, xe, ys, ye)?;
				count += secs.len();
				res.append(&mut secs);

				if sr.x >= 10
					&& sr.x <= (w.header.width - 10)
					&& sr.y >= 10 && sr.y <= (w.header.height - 10)
				{
					let (xs, xe, ys, ye) = get_sections_near(sr.x, sr.y, x_max - 1, y_max - 1);
					let mut secs = c.encode_sections(&w, xs, xe, ys, ye)?;
					count += secs.len();
					res.append(&mut secs);
				}

				res.insert(
					1,
					Message::SpawnResponse(SpawnResponse {
						status: count as i32,
						text: Text::Key("LegacyInterface.44".to_owned(), vec![]),
						flags: 0,
					}),
				);

				// if (flag4) {
				//   for (int sectionX = x1; sectionX <= num13; ++sectionX) {
				//     for (int sectionY = y1; sectionY <= num14; ++sectionY)
				//       NetMessage.SendSection(this.whoAmI, sectionX, sectionY);
				//   }
				// }

				// for (int index10 = 0; index10 < portalSections.Count; ++index10)
				//   NetMessage.SendSection(this.whoAmI, portalSections[index10].X, portalSections[index10].Y);

				// for (int number4 = 0; number4 < 400; ++number4)
				// {
				// 	if (Main.item[number4].active)
				// 	{
				// 		NetMessage.TrySendData(21, this.whoAmI, number: number4);
				// 		NetMessage.TrySendData(22, this.whoAmI, number: number4);
				// 	}
				// }

				// Send all NPCs
				// todo: use real npc slots
				for (id, npc) in w.npcs.iter().enumerate() {
					res.push(Message::NPCInfo(NPCInfo {
						id: id as i16,
						position: npc.position.clone(),
						velocity: Vector2(0., 0.),
						target: 0,
						flags_1: 128,
						flags_2: 0,
						npc_ai: vec![],
						id_2: npc.id as i16,
						stats_scaled_for_n_players: None,
						strength_multiplier: None,
						life_len: None,
						life_i8: None,
						life_i16: None,
						life_i32: None,
						release_owner: None,
					}));
				}

				// todo: add actual projectile data
				// for (int number6 = 0; number6 < 1000; ++number6) {
				// 	if (Main.projectile[number6].active && (Main.projPet[Main.projectile[number6].type] || Main.projectile[number6].netImportant))
				// 		NetMessage.TrySendData(27, this.whoAmI, number: number6);
				// }

				for (i, &kc) in w.header.npc_kill_counts.iter().enumerate() {
					if i >= 290 {
						break;
					}
					res.push(Message::KillCount(KillCount {
						id: i as i16,
						amount: kc,
					}))
				}

				res.push(Message::WorldTotals(WorldTotals {
					good: 0,
					evil: 0,
					blood: 0,
				}));

				// NetMessage.TrySendData(103); Message::MoonlordCountdown

				res.push(Message::PillarShieldStrengths(PillarShieldStrengths {
					nebula: 0,
					solar: 0,
					stardust: 0,
					vortex: 0,
				}));

				// todo: implement NPC.SetWorldSpecificMonstersByWorldID and UnifiedRandom or my own random gen
				res.push(Message::MonsterTypes([506, 506, 499, 495, 494, 495]));

				res.push(Message::PlayerSyncDone);

				// Main.BestiaryTracker.OnPlayerJoining(this.whoAmI);
				// CreativePowerManager.Instance.SyncThingsToJoiningPlayer(this.whoAmI);
				// Main.PylonSystem.OnPlayerJoining(this.whoAmI);

				res
				// vec![Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.1".to_owned()))]
			}
			Message::PlayerSpawnRequest(mut psr) => {
				psr.sanitize(src as u8);

				if client.state != ConnectionState::DetailsReceived
					&& client.state != ConnectionState::Complete
				{
					return Ok(vec![Message::ConnectionRefuse(Text::Key(
						"LegacyMultiplayer.1".to_owned(),
						vec![],
					))]);
				}

				tx.send((Message::PlayerSpawnRequest(psr), Some(src)))?;
				if client.state == ConnectionState::Complete {
					return Ok(vec![]);
				}

				client.state = ConnectionState::Complete;
				// if (NetMessage.DoesPlayerSlotCountAsAHost(this.whoAmI))
				//   NetMessage.TrySendData(139, this.whoAmI, number: this.whoAmI, number2: ((float) flag5.ToInt()));

				let world = self.world.read().await;

				vec![
					Message::AnglerQuest(AnglerQuest {
						id: world.header.angler_quest as u8,
						finished: false,
					}),
					Message::PlayerSpawnResponse,
				]
			}
			// This message just gets broadcasted
			Message::PlayerPickTile(mut ppt) => {
				ppt.sanitize(src as u8);
				tx.send((Message::PlayerPickTile(ppt), Some(src)))?;
				vec![]
			}
			Message::UpdateTile(ppt) => {
				// dbg!(&ppt);
				if ppt.action == 0 && ppt.target_type == 0 {
					let world = self.world.read().await;
					let tile = &world.tiles[ppt.x as usize][ppt.y as usize];
					let i = tile.get_dropped_item(client, &world, &mut 0, &mut 0, &mut 0);
					if i > 0 {
						tx.send((
							Message::DropItem(DropItem {
								id: 0,
								position: Vector2(ppt.x as f32 * TILE, ppt.y as f32 * TILE),
								velocity: Vector2(random::<f32>() * 6. - 3., (random::<f32>() * -2.5) - 1.5),
								item_id: i,
								own_ignore: false,
								prefix: 0,
								stack: 1,
							}),
							None,
						))?;
					}
				}
				tx.send((Message::UpdateTile(ppt), Some(src)))?;
				vec![]
			}
			Message::PlayerReserveItem(mut pri) => {
				pri.sanitize(src as u8);
				tx.send((Message::PlayerReserveItem(pri), None))?;
				vec![]
			}
			Message::PlayerAction(mut pa) => {
				if pa.selected_item != client.selected_item {
					dbg!(client.items.iter().skip(item_slots::ARMOR_LOADOUT_0_START).take(3).collect::<Vec<&Option<PlayerItemSlot>>>());
					dbg!(client.items.iter().skip(item_slots::ARMOR_LOADOUT_1_START).take(3).collect::<Vec<&Option<PlayerItemSlot>>>());
					dbg!(client.items.iter().skip(item_slots::ARMOR_LOADOUT_2_START).take(3).collect::<Vec<&Option<PlayerItemSlot>>>());
				}
				client.selected_item = pa.selected_item;
				pa.sanitize(src as u8);

				let w = self.world.read().await;
				let c = client;

				let x_max = get_section_x(w.header.width as usize) - 1;
				let y_max = get_section_y(w.header.height as usize) - 1;

				let x = get_section_x((pa.position.0 / TILE) as usize);
				let y = get_section_y((pa.position.1 / TILE) as usize);

				let xs = max(x - 1, 0);
				let xe = min(x + 1, x_max);
				let ys = max(y - 1, 0);
				let ye = min(y + 1, y_max);
				tx.send((Message::PlayerAction(pa), Some(src)))?;

				let sec = c.encode_sections(&w, xs, xe, ys, ye)?;
				if sec.is_empty() {
					vec![]
				} else {
					sec
				}
			}
			// Just gets broadcasted
			Message::PlayInstrument(mut pi) => {
				pi.sanitize(src as u8);
				tx.send((Message::PlayInstrument(pi), Some(src)))?;
				vec![]
			}
			// Server does nothing
			Message::InventorySynced => vec![],
			Message::Custom(code, buf) => {
				println!("Custom ({}): {:?}", code, buf);
				vec![]
			}
			pkt => {
				println!("Not yet implemented packet: {:?}", pkt);
				vec![]
			}
		})
	}
}
