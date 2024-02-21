use std::cmp::{max, min};
use std::io::Write;
use std::net::SocketAddr;
use std::pin::Pin;
use flate2::write::ZlibEncoder;
use flate2::{Compress, Compression};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{Result, AsyncReadExt};
use tokio::sync::{Mutex, RwLock, broadcast};
use tokio::select;
use std::sync::Arc;

use crate::binary::writer::Writer;
use crate::network::messages::{self, Sanitize, Message, DropItem, ConnectionApprove, WorldHeader, SpawnResponse, NPCInfo, KillCount, WorldTotals, PillarShieldStrengths, MonsterTypes, AnglerQuest};
use crate::binary::types::{Text, TextMode, Vector2};
use crate::world::types::{EntityExtra, Liquid, Tile, World};
use crate::network::utils::{flags, get_section_x, get_section_y};

use super::utils::{get_tile_x_end, get_tile_x_start, get_tile_y_end, get_tile_y_start};

const GAME_VERSION: &str = "Terraria279";
const MAX_CLIENTS: usize = 256;
const MAX_NAME_LEN: usize = 20;
const TILE: f32 = 16.;

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum ConnectionState {
	New,
	PendingAuthentication,
	Authenticated,
	DetailsReceived,
	Complete,
}

const MAX_INVENTORY_SLOTS: usize = 350;

pub struct Client {
	pub addr: SocketAddr,
	pub state: ConnectionState,
	pub uuid: Option<String>,
	pub details: Option<messages::PlayerDetails>,
	pub health: Option<messages::PlayerHealth>,
	pub mana: Option<messages::PlayerMana>,
	pub buffs: Option<messages::PlayerBuffs>,
	pub loadout: Option<messages::PlayerLoadout>,
	pub inventory: Arc<Mutex<[Option<messages::PlayerInventorySlot>; MAX_INVENTORY_SLOTS]>>,
	pub loaded_sections: Vec<Vec<bool>>,
}

impl Client {
	fn new(addr: SocketAddr, width: usize, height: usize) -> Self {
		// https://github.com/rust-lang/rust/issues/44796#issuecomment-967747810
		const INIT_SLOT_NONE: Option<messages::PlayerInventorySlot> = None;

		Self {
			addr,
			state: ConnectionState::New,
			details: None,
			uuid: None,
    	health: None,
			buffs: None,
			mana: None,
			loadout: None,
			inventory: Arc::new(Mutex::new([INIT_SLOT_NONE; MAX_INVENTORY_SLOTS])),
			loaded_sections: vec![vec![false; height]; width],
		}
	}
}

pub struct Server {
	pub world: RwLock<World>,
	pub password: RwLock<String>,
	pub clients: Arc<Mutex<[Option<Client>; MAX_CLIENTS]>>,
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
			clients: Arc::new(Mutex::new([INIT_CLIENT_NONE; MAX_CLIENTS])),
			broadcast: tx,
		}
	}

	pub async fn listen(self, address: &str) -> Result<()> {
		let listener = TcpListener::bind(address).await?;
		let arc = Arc::new(self);

		loop {
			let (mut stream, addr) = listener.accept().await?;
			let rc = arc.clone();
			tokio::spawn(async move {
				rc.accept(&mut stream, addr).await
			});
		}
	}

	async fn accept(&self, stream: &mut TcpStream, addr: SocketAddr) {
		let (mut rh, mut wh) = stream.split();
		let mut tx = self.broadcast.clone();
		let mut rx = self.broadcast.subscribe();

		// check if a player slot is available
		let src = {
			let mut clients = self.clients.lock().await;
			let Some(id) = clients.iter().position(Option::is_none) else {
				Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "CLI.ServerIsFull".to_owned())).write(Pin::new(&mut wh)).await.unwrap();
				return
			};
			let world = self.world.read().await;
			clients[id] = Some(Client::new(addr, get_section_x(world.header.width as usize), get_section_y(world.header.height as usize)));
			id
		};

		loop {
			let mut length = [0u8; 2];
			select! {
				read_result = rh.read(&mut length) => {
					// Player disconnected
					if read_result.is_err() || read_result.unwrap() == 0 {
						self.clients.lock().await[src] = None;
						return;
					}

					let length = u16::from_le_bytes(length);
					if length < 2 {
						continue;
					}

					let mut buffer = vec![0u8; length as usize - 2];
					let read_result = rh.read_exact(&mut buffer).await;
					if read_result.is_err() || read_result.unwrap() == 0 {
						self.clients.lock().await[src] = None;
						return;
					}

					let to_send = self.handle_message(Message::from(buffer), src, &mut tx).await;
					for msg in to_send {
						msg.write(Pin::new(&mut wh)).await.unwrap();
					}
				}
				content = rx.recv() => {
					let (content, ignore_id) = content.unwrap();
					if ignore_id.map_or(true, |id| id != src) {
						// dbg!(&content);
						content.write(Pin::new(&mut wh)).await.unwrap();
					}
				}
			}
		}
	}

	async fn handle_message(&self, msg: Message, src: usize, tx: &mut broadcast::Sender<(Message, Option<usize>)>) -> Vec<Message> {
		let mut clients = self.clients.lock().await;

		match msg {
			// The client sends their version and if it matches the server version, we send ConnectionApprove if there is not password and PasswordRequest if there is a password
			// If their version does not match, refuse connection
			Message::VersionIdentifier(version) => {
				if clients[src].as_ref().unwrap().state != ConnectionState::New {
					return vec![]
				}

				if version == GAME_VERSION {
					let password = self.password.read().await;
					if password.is_empty() {
						clients[src].as_mut().unwrap().state = ConnectionState::Authenticated;
						vec![Message::ConnectionApprove(ConnectionApprove {
							client_id: src as u8,
							flag: false,
						})]
					} else {
						clients[src].as_mut().unwrap().state = ConnectionState::PendingAuthentication;
						vec![Message::PasswordRequest]
					}
				} else {
					println!("Player tried joining with unsupported version {}", version);
					vec![Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.4".to_owned()))]
				}
			}
			// The client sends back the password. If it's correct, we may send back their dedicated user id
			// Don't know what the "false" is but that's what the source code does
			// If the password is wrong, refuse connection
			Message::PasswordResponse(pass) => {
				let password = self.password.read().await;
				if pass == password.as_str() {
					clients[src].as_mut().unwrap().state = ConnectionState::Authenticated;
					vec![Message::ConnectionApprove(ConnectionApprove {
						client_id: src as u8,
						flag: false,
					})]
				} else {
					vec![Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.1".to_owned()))]
				}
			}
			// The player sends their character's UUID. Terraria seems to do nothing with it so let's just store it
			Message::UUID(uuid) => {
				clients[src].as_mut().unwrap().uuid = Some(uuid);
				vec![]
			}
			// If another player already exists with the same name, refuse this player
			// The player sends character details upon first join. Store it
			// Broadcast this player to all other players
			Message::PlayerDetails(mut pd) => {
				if clients[src].as_ref().unwrap().state != ConnectionState::Authenticated {
					return vec![Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.1".to_owned()))]
				}

				let exists_same_name = clients
					.iter()
					.any(
						|c_opt| c_opt.as_ref().map_or(false,
							|c| c.details.as_ref().map_or(false, |d| d.name == pd.name)));
				if exists_same_name {
					// TODO: support NetworkText.FromKey substitutions
					return vec![Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.5".to_owned()))]
				}

				if pd.name.len() > MAX_NAME_LEN {
					return vec![Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "Net.NameTooLong".to_owned()))]
				}

				if pd.name.is_empty() {
					return vec![Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "Net.EmptyName".to_owned()))]
				}

				// TODO: compare client difficulty with world difficulty

				pd.sanitize(src as u8);
				tx.send((Message::PlayerDetails(pd.clone()), Some(src))).unwrap();
				let c = clients[src].as_mut().unwrap();
				c.details = Some(pd);
				c.state = ConnectionState::DetailsReceived;
				vec![]
			}
			Message::PlayerHealth(mut ph) => {
				ph.sanitize(src as u8);
				tx.send((Message::PlayerHealth(ph.clone()), Some(src))).unwrap();
				clients[src].as_mut().unwrap().health = Some(ph);
				vec![]
			}
			// Doesn't get broadcast
			Message::PlayerMana(mut pm) => {
				pm.sanitize(src as u8);
				clients[src].as_mut().unwrap().mana = Some(pm);
				vec![]
			}
			Message::PlayerBuffs(mut pb) => {
				pb.sanitize(src as u8);
				tx.send((Message::PlayerBuffs(pb.clone()), Some(src))).unwrap();
				clients[src].as_mut().unwrap().buffs = Some(pb);
				vec![]
			}
			Message::PlayerLoadout(mut psl) => {
				psl.sanitize(src as u8);
				tx.send((Message::PlayerLoadout(psl.clone()), Some(src))).unwrap();
				clients[src].as_mut().unwrap().loadout = Some(psl);
				vec![]
			}
			Message::PlayerInventorySlot(mut pis) => {
				pis.sanitize(src as u8);
				let idx = pis.slot_id as usize;
				if idx < MAX_INVENTORY_SLOTS {
					tx.send((Message::PlayerInventorySlot(pis.clone()), Some(src))).unwrap();
					clients[src].as_mut().unwrap().inventory.as_ref().lock().await[idx] = Some(pis);
				}
				vec![]
			}
			Message::WorldRequest => {
				vec![self.get_msg_world_header().await]
				// todo: Main.SyncAnInvasion
			}
			Message::SpawnRequest(_sr) => {
				if clients[src].as_ref().unwrap().state != ConnectionState::DetailsReceived {
					return vec![Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.1".to_owned()))]
				}

				let w = self.world.read().await;
				let c = clients[src].as_mut().unwrap();
				let mut res = vec![self.get_msg_world_header().await];

				let max_sec_x = get_section_x(w.header.width as usize);
				let max_sec_y = get_section_y(w.header.height as usize);

				let sec_x_start  = max(get_section_x(w.header.spawn_tile_x as usize) - 2, 0);
				let sec_y_start  = max(get_section_y(w.header.spawn_tile_y as usize) - 1, 0);
				let sec_x_end = min(sec_x_start + 5, max_sec_x);
				let sec_y_end = min(sec_y_start + 3, max_sec_y);

				let sec_count = (sec_x_end - sec_x_start) * (sec_y_end - sec_y_start);
				res.push(Message::SpawnResponse(SpawnResponse {
					status: sec_count as i32,
					text: Text(TextMode::LocalizationKey, "LegacyInterface.44".to_owned()),
					flags: 0,
				}));

				// List<Point> dontInclude = new List<Point>();
				// for (int x2 = sec_x_start; x2 < sec_x_end; ++x2)
				// {
				// 	for (int y2 = sec_y_start; y2 < sec_y_end; ++y2)
				// 		dontInclude.Add(new Point(x2, y2));
				// }

				// for (int x = sec_x_start; x <= send_x_end; ++x)
				// {
				// 	for (int y = sec_y_start; y <= send_y_end; ++y)
				// 	{
				// 		if (x < sec_x_start || x >= sec_x_end || y < sec_y_start || y >= sec_y_end)
				// 		{
				// 			dontInclude.Add(new Point(x, y));
				// 			++sec_count;
				// 		}
				// 	}
				// }

				// List<Point> portalSections;
				// PortalHelper.SyncPortalsOnPlayerJoin(this.whoAmI, 1, dontInclude, out portalSections);
				// sec_count += portalSections.Count;

				for x in sec_x_start..sec_x_end {
					for y in sec_y_start..sec_y_end {
						if c.loaded_sections[x][y] {
							continue;
						}

						c.loaded_sections[x][y] = true;
						res.push(self.get_msg_section(x, y).await)
					}
				}

				// if !(sr.x < 10 || sr.x > (w.header.width - 10) || sr.y < 10 || sr.y > (w.header.height - 10)) {
				// 	let sec_x_start  = max(get_section_x(sr.x) - 2, 0);
				// 	let sec_y_start  = max(get_section_y(sr.y) - 1, 0);
				// 	let sec_x_end = min(sec_x_start + 5, max_sec_x);
				// 	let sec_y_end = min(sec_y_start + 3, max_sec_y);

				// 	for x in sec_x_start..sec_x_end {
				// 		for y in sec_y_start..sec_y_end {
				// 			res.push(self.get_msg_section(x, y).await)
				// 		}
				// 	}
				// }

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
				for npc in &w.npcs {
					res.push(Message::NPCInfo(NPCInfo {
						id: npc.id as i16,
						position: npc.position.clone(),
						velocity: Vector2(0., 0.),
						target: 0,
						flags_1: 0,
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
					}))
				}

				// for (int number6 = 0; number6 < 1000; ++number6) {
				// 	if (Main.projectile[number6].active && (Main.projPet[Main.projectile[number6].type] || Main.projectile[number6].netImportant))
				// 		NetMessage.TrySendData(27, this.whoAmI, number: number6);
				// }

				for (i, &kc) in w.header.npc_kill_counts.iter().enumerate() {
					if i >= 290 { break }
					res.push(Message::KillCount(KillCount {
						id: i as i16,
						amount: kc,
					}))
				}

				res.push(Message::WorldTotals(WorldTotals {
					good: 0,
					evil: 6,
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
				res.push(Message::MonsterTypes(MonsterTypes {
					all: [506, 506, 499, 495, 494, 495],
				}));

				res.push(Message::PlayerSyncDone);

				// Main.BestiaryTracker.OnPlayerJoining(this.whoAmI);
				// CreativePowerManager.Instance.SyncThingsToJoiningPlayer(this.whoAmI);
				// Main.PylonSystem.OnPlayerJoining(this.whoAmI);

				res
				// vec![Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.1".to_owned()))]
			}
			Message::PlayerSpawnRequest(mut psr) => {
				psr.sanitize(src as u8);
				let client = clients[src].as_mut().unwrap();

				if client.state != ConnectionState::DetailsReceived && client.state != ConnectionState::Complete {
					return vec![Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.1".to_owned()))]
				}

				tx.send((Message::PlayerSpawnRequest(psr), Some(src))).unwrap();
				if client.state == ConnectionState::Complete {
					return vec![];
				}

				client.state = ConnectionState::Complete;
				// if (NetMessage.DoesPlayerSlotCountAsAHost(this.whoAmI))
				//   NetMessage.TrySendData(139, this.whoAmI, number: this.whoAmI, number2: ((float) flag5.ToInt()));

				let world = self.world.read().await;

				vec![
					Message::AnglerQuest(AnglerQuest { id: world.header.angler_quest as u8, finished: false }),
					Message::PlayerSpawnResponse,
				]
			}
			// This message just gets broadcasted
			Message::PlayerPickTile(mut ppt) => {
				ppt.sanitize(src as u8);
				tx.send((Message::PlayerPickTile(ppt), Some(src))).unwrap();
				vec![]
			}
			Message::UpdateTile(ppt) => {
				if ppt.action == 0 && ppt.target_type == 0 {
					let world = self.world.read().await;
					let tile = &world.tiles[ppt.x as usize][ppt.y as usize];
					tx.send((Message::DropItem(DropItem {
						id: 0,
						position: Vector2(ppt.x as f32 * TILE, ppt.y as f32 * TILE),
						velocity: Vector2(0., 1.),
						item_id: tile.id,
						own_ignore: false,
						prefix: 0,
						stack: 1,
					}), None)).unwrap();
				}
				tx.send((Message::UpdateTile(ppt), Some(src))).unwrap();
				vec![]
			}
			Message::PlayerKeepItem(mut pki) => {
				pki.sanitize(src as u8);
				tx.send((Message::PlayerKeepItem(pki), None)).unwrap();
				vec![]
			}
			Message::PlayerAction(mut pa) => {
				pa.sanitize(src as u8);
				dbg!(&pa);

				let w = self.world.read().await;
				let c = clients[src].as_mut().unwrap();

				let max_sec_x = get_section_x(w.header.width as usize);
				let max_sec_y = get_section_y(w.header.height as usize);

				let sec_x_start  = max(get_section_x((pa.position.0 / TILE) as usize) - 1, 0);
				let sec_y_start  = max(get_section_y((pa.position.1 / TILE) as usize) - 1, 0);
				let sec_x_end = min(sec_x_start + 1, max_sec_x);
				let sec_y_end = min(sec_y_start + 1, max_sec_y);
				let mut tiles = vec![];
				for x in sec_x_start..sec_x_end {
					for y in sec_y_start..sec_y_end {
						if c.loaded_sections[x][y] {
							continue;
						}

						c.loaded_sections[x][y] = true;
						tiles.push(self.get_msg_section(x, y).await)
					}
				}

				tx.send((Message::PlayerAction(pa), Some(src))).unwrap();
				tiles
			}
			Message::Custom(code, buf) => {
				println!("Custom ({}): {:?}", code, buf);
				vec![]
			}
			pkt => {
				println!("Not yet implemented packet: {:?}", pkt);
				vec![]
			}
		}
	}

	async fn get_msg_section(&self, sec_x: usize, sec_y: usize) -> Message {
		let x_start = get_tile_x_start(sec_x);
		let y_start = get_tile_y_start(sec_y);
		let x_end = get_tile_x_end(sec_x);
		let y_end = get_tile_y_end(sec_y);

		// todo: optimize this to reduce data copying

		let mut w = Writer::new(0);
		w.write_i32(x_start as i32);
		w.write_i32(y_start as i32);
		w.write_i16((x_end - x_start) as i16);
		w.write_i16((y_end - y_start) as i16);

		let world = &self.world.read().await;
		let tiles = &world.tiles;
		let importance = &world.format.importance;

		let mut last_tile = &Tile::default();
		let mut repeat_count: u16 = 0;

		let mut chest_tiles = vec![];
		let mut sign_tiles = vec![];
		let mut entity_tiles = vec![];

		let mut buf = [0u8; 16];
		let mut i = 0;
		let mut j = 0;
		let mut h_1 = 0;

		for y in y_start..y_end {
			for x in x_start..x_end {
				let tile = &tiles[x as usize][y as usize];

				// todo: ensure isTheSameAs is like PartialEq
				// todo: automate this to use TileID.Sets.AllowsSaveCompressionBatching
				if tile == last_tile && tile.id != 520 && tile.id != 423 {
					repeat_count += 1;
					continue;
				}

				if !(y == y_start && x == x_start) {
					if repeat_count > 0 {
						buf[i] = repeat_count as u8;
						i += 1;
						if repeat_count > u8::MAX as u16 {
							h_1 |= 128;
							buf[i] = (repeat_count >> 8) as u8;
							i += 1
						} else {
							h_1 |= 64;
						}
					}

					buf[j] = h_1;
					w.write_bytes(buf[j..i].to_vec());
					repeat_count = 0;
				}

				i = 4;
				let mut h_2 = 0;
				let mut h_3 = 0;
				let mut h_4 = 0;
				h_1 = 0;

				if tile.active {
					h_1 |= 2;
					buf[i] = tile.id as u8;
					i += 1;
					if tile.id > u8::MAX as i16 {
						buf[i] = (tile.id >> 8) as u8;
						i += 1;
						h_1 |= 32;
					}

					if importance[tile.id as usize] {
						let fx = tile.frame_x;
						let fy = tile.frame_y;
						let is_chest = match tile.id {
							21 | 467 => fx % 36 == 0 && fy % 36 == 0,
							88 => fx % 54 == 0 && fy % 36 == 0,
							_ => false,
						};
						if is_chest {
							chest_tiles.push((x, y));
						} else {
							let is_sign = match tile.id {
								55 | 85 | 425 | 573 => fx % 36 == 0 && fy % 36 == 0,
								_ => false,
							};
							if is_sign {
								sign_tiles.push((x, y));
							} else {
								let is_entity = match tile.id {
									378 | 395 | 470 => fx % 36 == 0 && fy == 0,
									520 => fx % 18 == 0 && fy == 0,
									471 | 475 => fx % 54 == 0 && fy == 0,
									597 => fx % 54 == 0 && fy % 72 == 0,
									_ => false,
								};
								if is_entity {
									entity_tiles.push((x, y));
								}
							}
						}

						[buf[i], buf[i + 1]] = tile.frame_x.to_le_bytes();
						i += 2;
						[buf[i], buf[i + 1]] = tile.frame_y.to_le_bytes();
						i += 2;
					}

					if tile.color > 0 {
						h_3 |= 8;
						buf[i] = tile.color;
						i += 1;
					}
				}

				if tile.wall > 0 {
					h_1 |= 4;
					buf[i] = tile.wall as u8;
					i += 1;

					if tile.wall_color > 0 {
						h_3 |= 16;
						buf[i] = tile.wall_color as u8;
						i += 1;
					}
				}

				if tile.liquid_header > 0 {
					let (f_1, f_3) = match tile.liquid {
						Liquid::Shimmer => (8, 128),
						Liquid::Lava => (16, 0),
						Liquid::Honey => (24, 0),
						_ => (8, 0),
					};
					h_1 |= f_1;
					h_3 |= f_3;
					buf[i] = tile.liquid_header;
					i += 1;
				}

				if tile.wire_1 {
					h_4 |= 2;
				}
				if tile.wire_2 {
					h_4 |= 4;
				}
				if tile.wire_3 {
					h_4 |= 8;
				}
				if tile.half_brick {
					h_4 |= 16;
				} else if tile.slope > 0 {
					h_4 |= (tile.slope + 1) << 4;
				}
				if tile.actuator {
					h_3 |= 2;
				}
				if tile.in_active {
					h_3 |= 4;
				}
				if tile.wire_4 {
					h_3 |= 32;
				}

				if tile.wall > u8::MAX as u16 {
					h_3 |= 64;
					buf[i] = (tile.wall >> 8) as u8;
					i += 1;
				}

				if tile.invisible_block {
					h_2 |= 2;
				}
				if tile.invisible_wall {
					h_2 |= 4;
				}
				if tile.fullbright_block {
					h_2 |= 8;
				}
				if tile.fullbright_wall {
					h_2 |= 16;
				}
				j = 3;
				if h_2 > 0 {
					h_3 |= 1;
					buf[j] = h_2;
					j -= 1;
				}
				if h_3 > 0 {
					h_4 |= 1;
					buf[j] = h_3;
					j -= 1;
				}
				if h_4 > 0 {
					h_1 |= 1;
					buf[j] = h_4;
					j -= 1;
				}

				last_tile = tile;
			}
		}

		if repeat_count > 0 {
			buf[i] = repeat_count as u8;
			i += 1;
			if repeat_count > u8::MAX as u16 {
				h_1 |= 128;
				buf[i] = (repeat_count >> 8) as u8;
				i += 1
			} else {
				h_1 |= 64;
			}
		}

		buf[j] = h_1;
		w.write_bytes(buf[j..i].to_vec());

		// todo: send npcs, signs, and portals
		w.write_i16(chest_tiles.len() as i16);
		for (x, y) in chest_tiles {
			let (i, chest) = world.chests.iter().enumerate().find(|(_, c)| c.x as usize == x && c.y as usize == y).unwrap();
			w.write_i16(i as i16);
			w.write_i16(x as i16);
			w.write_i16(y as i16);
			w.write_string(chest.name.clone());
		}

		w.write_i16(sign_tiles.len() as i16);
		for (x, y) in sign_tiles {
			let (i, sign) = world.signs.iter().enumerate().find(|(_, s)| s.x as usize == x && s.y as usize == y).unwrap();
			w.write_i16(i as i16);
			w.write_i16(x as i16);
			w.write_i16(y as i16);
			w.write_string(sign.text.clone());
		}

		w.write_i16(entity_tiles.len() as i16);
		for (x, y) in entity_tiles {
			let entity = world.entities.iter().find(|c| c.x == x as i16 && c.y == y as i16).unwrap();
			match &entity.entity {
				EntityExtra::Dummy { npc } => {
					w.write_byte(0);
					w.write_i32(entity.id);
					w.write_i16(entity.x);
					w.write_i16(entity.y);
					w.write_i16(*npc);
				},
				EntityExtra::ItemFrame(frame) => {
					w.write_byte(1);
					w.write_i32(entity.id);
					w.write_i16(entity.x);
					w.write_i16(entity.y);
					w.write_i16(frame.id);
					w.write_byte(frame.prefix);
					w.write_i16(frame.stack);
				},
				EntityExtra::LogicSensor { logic_check, on } => {
					w.write_byte(2);
					w.write_i32(entity.id);
					w.write_i16(entity.x);
					w.write_i16(entity.y);
					w.write_byte(*logic_check);
					w.write_bool(*on);
				},
				EntityExtra::DisplayDoll(doll) => {
					w.write_byte(3);
					w.write_i32(entity.id);
					w.write_i16(entity.x);
					w.write_i16(entity.y);

					let mut item_flags = 0;
					for (i, item) in doll.items.iter().rev().enumerate() {
						if item.id != 0 {
							item_flags |= 1;
						}
						if i != 0 {
							item_flags <<= 1;
						}
					}
					w.write_byte(item_flags);

					let mut dye_flags = 0;
					for (i, dye) in doll.dyes.iter().rev().enumerate() {
						if dye.id != 0 {
							dye_flags |= 1;
						}
						if i != 0 {
							dye_flags <<= 1;
						}
					}
					w.write_byte(dye_flags);

					for item in &doll.items {
						if item.id != 0 {
							w.write_i16(item.id);
							w.write_byte(item.prefix);
							w.write_i16(item.stack);
						}
					}

					for dye in &doll.dyes {
						if dye.id != 0 {
							w.write_i16(dye.id);
							w.write_byte(dye.prefix);
							w.write_i16(dye.stack);
						}
					}
				},
				EntityExtra::WeaponsRack(rack) => {
					w.write_byte(4);
					w.write_i32(entity.id);
					w.write_i16(entity.x);
					w.write_i16(entity.y);
					w.write_i16(rack.id);
					w.write_byte(rack.prefix);
					w.write_i16(rack.stack);
				},
				EntityExtra::HatRack(rack) => {
					w.write_byte(5);
					w.write_i32(entity.id);
					w.write_i16(entity.x);
					w.write_i16(entity.y);

					let mut flags = 0;
					for item in rack.items.iter().rev() {
						if item.id != 0 {
							flags |= 1;
						}
						flags <<= 1;
					}
					for (i, dye) in rack.dyes.iter().rev().enumerate() {
						if dye.id != 0 {
							flags |= 1;
						}
						if i != 0 {
							flags <<= 1;
						}
					}
					w.write_byte(flags);

					for item in &rack.items {
						if item.id != 0 {
							w.write_i16(item.id);
							w.write_byte(item.prefix);
							w.write_i16(item.stack);
						}
					}

					for dye in &rack.dyes {
						if dye.id != 0 {
							w.write_i16(dye.id);
							w.write_byte(dye.prefix);
							w.write_i16(dye.stack);
						}
					}
				},
				EntityExtra::FoodPlatter(platter) => {
					w.write_byte(6);
					w.write_i32(entity.id);
					w.write_i16(entity.x);
					w.write_i16(entity.y);
					w.write_i16(platter.id);
					w.write_byte(platter.prefix);
					w.write_i16(platter.stack);
				},
				EntityExtra::TeleportationPylon => {
					w.write_byte(7);
					w.write_i32(entity.id);
					w.write_i16(entity.x);
					w.write_i16(entity.y);
				},
			}
		}

		let compress = Compress::new_with_window_bits(Compression::default(), false, 15);
		let mut out = ZlibEncoder::new_with_compress(vec![], compress);
		out.write_all(&w.buf[3..]).unwrap();

		Message::Custom(10, out.finish().unwrap())
	}

	async fn get_msg_world_header(&self) -> Message {
		let h = &self.world.read().await.header;

		Message::WorldHeader(WorldHeader {
			time: 0,
			time_flags: flags( h.temp_day_time, h.temp_blood_moon, h.temp_eclipse, false, false, false, false, false),
			moon_phase: h.temp_moon_phase as u8,
			width: h.width as i16,
			height: h.height as i16,
			spawn_x: h.spawn_tile_x as i16,
			spawn_y: h.spawn_tile_y as i16,
			world_surface: h.world_surface as i16,
			rock_layer: h.rock_layer as i16,
			id: h.id,
			name: h.name.clone(),
			game_mode: h.game_mode.clone() as u8,
			uuid: h.uuid.unwrap(),
			worldgen_version: h.worldgen_version,
			moon_type: h.moon_type as u8,
			bg_0: h.bg[0],
			bg_10: h.bg[10],
			bg_11: h.bg[11],
			bg_12: h.bg[12],
			bg_1: h.bg[1],
			bg_2: h.bg[2],
			bg_3: h.bg[3],
			bg_4: h.bg[4],
			bg_5: h.bg[5],
			bg_6: h.bg[6],
			bg_7: h.bg[7],
			bg_8: h.bg[8],
			bg_9: h.bg[9],
			ice_back_style: h.ice_back_style as u8,
			jungle_back_style: h.jungle_back_style as u8,
			hell_back_style: h.hell_back_style as u8,
			wind_speed_target: h.wind_speed_target,
			num_clouds: h.num_clouds as u8,
			tree_x: h.tree_x,
			tree_style: h.tree_style.iter().map(|n| *n as u8).collect::<Vec<u8>>().try_into().unwrap_or([0; 4]),
			cave_back_x: h.cave_back_x,
			cave_back_style: h.cave_back_style.iter().map(|n| *n as u8).collect::<Vec<u8>>().try_into().unwrap_or([0; 4]),
			tree_top_variations: h.tree_top_variations.iter().map(|n| *n as u8).collect::<Vec<u8>>().try_into().unwrap_or([0; 13]),
			max_raining: h.temp_max_rain,
			flags: [
				// todo: support for server-side characters
				flags(h.smashed_shadow_orb, h.downed_boss_1, h.downed_boss_2, h.downed_boss_3, h.hard_mode, h.downed_clown, false, h.downed_plant_boss),
				// todo: pumpkinMoon and snowMoon
				flags(h.downed_mech_boss_1, h.downed_mech_boss_2, h.downed_mech_boss_3, h.downed_mech_boss_any, h.cloud_bg_active == 1., h.has_crimson, false, false),
				// todo: int num7 = bitsByte7[2] ? 1 : 0;
				flags(false, h.fast_forward_time_to_dawn, false, h.downed_slime_king, h.downed_queen_bee, h.downed_fishron, h.downed_martians, h.downed_ancient_cultist),
				// todo: BirthdayParty
				flags(h.downed_moonlord, h.downed_halloween_king, h.downed_halloween_tree, h.downed_christmas_ice_queen, h.downed_christmas_santank, h.downed_christmas_tree, h.downed_golem_boss, false),
				// todo: DD2Event.Ongoing
				flags(h.downed_pirates, h.downed_frost, h.downed_goblins, h.temp_sandstorm_happening, false, h.downed_dd2_invasion_t1, h.downed_dd2_invasion_t2, h.downed_dd2_invasion_t3),
				flags(h.combat_book_was_used, h.temp_lantern_night_manual, h.downed_tower_solar, h.downed_tower_vortex, h.downed_tower_nebula, h.downed_tower_stardust, h.force_halloween_for_today, h.force_xmas_for_today),
				// todo: freeCake, getGodWorld
				flags(h.bought_cat, h.bought_dog, h.bought_bunny, false, h.world_drunk, h.downed_empress_of_light, h.downed_queen_slime, false),
				flags(h.world_anniversary, h.world_dont_starve, h.downed_deerclops, h.world_not_the_bees, h.world_remix, h.unlocked_slime_blue_spawn, h.combat_book_volume_two_was_used, h.peddlers_satchel_was_used),
				flags(h.unlocked_slime_green_spawn, h.unlocked_slime_old_spawn, h.unlocked_slime_purple_spawn, h.unlocked_slime_rainbow_spawn, h.unlocked_slime_red_spawn, h.unlocked_slime_yellow_spawn, h.unlocked_slime_copper_spawn, h.fast_forward_time_to_dusk),
				flags(h.world_no_traps, h.world_zenith, h.unlocked_truffle_spawn, false, false, false, false, false),
			],
			sundial_cooldown: h.sundial_cooldown as u8,
			moondial_cooldown: h.moondial_cooldown,
			ore_tier_copper: h.ore_tier_copper as i16,
			ore_tier_iron: h.ore_tier_iron as i16,
			ore_tier_silver: h.ore_tier_silver as i16,
			ore_tier_gold: h.ore_tier_gold as i16,
			ore_tier_cobalt: h.ore_tier_cobalt as i16,
			ore_tier_mythril: h.ore_tier_mythril as i16,
			ore_tier_adamantite: h.ore_tier_adamantite as i16,
			invasion_type: h.invasion_type as i8,
			lobby_id: 0,
			sandstorm_intended_severity: h.temp_sandstorm_intended_severity,
		})
	}
}
