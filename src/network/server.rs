use std::net::SocketAddr;
use std::pin::Pin;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{Result, AsyncReadExt};
use tokio::sync::{Mutex, RwLock, broadcast};
use tokio::select;
use std::sync::Arc;

use crate::network::messages::{self, Sanitize, Message, ConnectionApprove};
use crate::binary::types::{Text, TextMode};

const GAME_VERSION: &str = "Terraria279";
const MAX_CLIENTS: usize = 256;
const MAX_NAME_LEN: usize = 20;

#[derive(PartialEq, Eq)]
pub enum ConnectionState {
	New,
	PendingAuthentication,
	Authenticated,
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
}

impl Client {
	fn new(addr: SocketAddr) -> Self {
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
		}
	}
}

pub struct Server {
	pub clients: Arc<Mutex<[Option<Client>; MAX_CLIENTS]>>,
	pub password: RwLock<String>,
	pub broadcast: broadcast::Sender<(Message<'static>, Option<usize>)>,
}

impl Server {
	pub fn new(password: &str) -> Server {
		// https://github.com/rust-lang/rust/issues/44796#issuecomment-967747810
		const INIT_CLIENT_NONE: Option<Client> = None;
    let (tx, _) = broadcast::channel(1024);

		Server {
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

	async fn accept(&self, stream: &mut TcpStream, addr: SocketAddr) -> Result<()> {
		let (mut rh, mut wh) = stream.split();
		let mut tx = self.broadcast.clone();
		let mut rx = self.broadcast.subscribe();

		// check if a player slot is available
		let src = {
			let mut clients = self.clients.lock().await;
			let Some(id) = clients.iter().position(Option::is_none) else {
				Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "CLI.ServerIsFull".to_owned())).write(Pin::new(&mut wh)).await.unwrap();
				return Ok(())
			};
			clients[id] = Some(Client::new(addr));
			id
		};

		loop {
			let mut length = [0u8; 2];
			select! {
				read_result = rh.read(&mut length) => {
					read_result?;
					let length = u16::from_le_bytes(length);

					if length < 2 {
						continue;
					}

					let mut buffer = vec![0u8; length as usize - 2];
					rh.read_exact(&mut buffer).await?;

					if let Some(msg) = self.handle_message(Message::from(buffer.as_slice()), src, &mut tx).await? {
						msg.write(Pin::new(&mut wh)).await.unwrap();
					}
				}
				content = rx.recv() => {
					let (content, ignore_id) = content.unwrap();
					if ignore_id.map_or(true, |id| id != src) {
						content.write(Pin::new(&mut wh)).await.unwrap();
					}
				}
			}
		}
	}

	async fn handle_message(&self, msg: Message<'_>, src: usize, tx: &mut broadcast::Sender<(Message<'static>, Option<usize>)>) -> Result<Option<Message<'static>>> {
		let mut clients = self.clients.lock().await;

		match msg {
			// The client sends their version and if it matches the server version, we send ConnectionApprove if there is not password and PasswordRequest if there is a password
			// If their version does not match, refuse connection
			Message::VersionIdentifier(version) => {
				if clients[src].as_ref().unwrap().state != ConnectionState::New {
					return Ok(None)
				}

				if version == GAME_VERSION {
					let password = self.password.read().await;
					if password.is_empty() {
						clients[src].as_mut().unwrap().state = ConnectionState::Authenticated;
						Ok(Some(Message::ConnectionApprove(ConnectionApprove {
							client_id: src as u8,
							flag: false,
						})))
					} else {
						clients[src].as_mut().unwrap().state = ConnectionState::PendingAuthentication;
						Ok(Some(Message::PasswordRequest))
					}
				} else {
					println!("Player tried joining with unsupported version {}", version);
					Ok(Some(Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.4".to_owned()))))
				}
			}
			// The client sends back the password. If it's correct, we may send back their dedicated user id
			// Don't know what the "false" is but that's what the source code does
			// If the password is wrong, refuse connection
			Message::PasswordResponse(pass) => {
				let password = self.password.read().await;
				if pass == password.as_str() {
					clients[src].as_mut().unwrap().state = ConnectionState::Authenticated;
					Ok(Some(Message::ConnectionApprove(ConnectionApprove {
						client_id: src as u8,
						flag: false,
					})))
				} else {
					Ok(Some(Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.1".to_owned()))))
				}
			}
			// The player sends their character's UUID. Terraria seems to do nothing with it so let's just store it
			Message::UUID(uuid) => {
				clients[src].as_mut().unwrap().uuid = Some(uuid);
				Ok(None)
			}
			// If another player already exists with the same name, refuse this player
			// The player sends character details upon first join. Store it
			// Broadcast this player to all other players
			Message::PlayerDetails(mut pd) => {
				if clients[src].as_ref().unwrap().state != ConnectionState::Authenticated {
					return Ok(Some(Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.1".to_owned()))))
				}

				if clients[src].as_ref().unwrap().state != ConnectionState::Complete {
					let exists_same_name = clients
						.iter()
						.any(
							|c_opt| c_opt.as_ref().map_or(false,
								|c| c.details.as_ref().map_or(false, |d| d.name == pd.name)));
					if exists_same_name {
						// TODO: support NetworkText.FromKey substitutions
						return Ok(Some(Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.5".to_owned()))))
					}
				}

				if pd.name.len() > MAX_NAME_LEN {
					return Ok(Some(Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "Net.NameTooLong".to_owned()))))
				}

				if pd.name.is_empty() {
					return Ok(Some(Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "Net.EmptyName".to_owned()))))
				}

				// TODO: compare client difficulty with world difficulty

				pd.sanitize(src as u8);
				tx.send((Message::PlayerDetails(pd.clone()), Some(src))).unwrap();
				clients[src].as_mut().unwrap().details = Some(pd);
				Ok(None)
			}
			Message::PlayerHealth(mut ph) => {
				ph.sanitize(src as u8);
				tx.send((Message::PlayerHealth(ph.clone()), Some(src))).unwrap();
				clients[src].as_mut().unwrap().health = Some(ph);
				Ok(None)
			}
			// Doesn't get broadcast
			Message::PlayerMana(mut pm) => {
				pm.sanitize(src as u8);
				clients[src].as_mut().unwrap().mana = Some(pm);
				Ok(None)
			}
			Message::PlayerBuffs(mut pb) => {
				pb.sanitize(src as u8);
				tx.send((Message::PlayerBuffs(pb.clone()), Some(src))).unwrap();
				clients[src].as_mut().unwrap().buffs = Some(pb);
				Ok(None)
			}
			Message::PlayerLoadout(mut psl) => {
				psl.sanitize(src as u8);
				tx.send((Message::PlayerLoadout(psl.clone()), Some(src))).unwrap();
				clients[src].as_mut().unwrap().loadout = Some(psl);
				Ok(None)
			}
			Message::PlayerInventorySlot(mut pis) => {
				pis.sanitize(src as u8);
				let idx = pis.slot_id as usize;
				if idx < MAX_INVENTORY_SLOTS {
					tx.send((Message::PlayerInventorySlot(pis.clone()), Some(src))).unwrap();
					clients[src].as_mut().unwrap().inventory.as_ref().lock().await[idx] = Some(pis);
				}
				Ok(None)
			}
			Message::Unknown(code, buf) => {
				println!("Unknown ({}): {:?}", code, buf);
				Ok(None)
			}
			pkt => {
				println!("Not yet implemented packet: {:?}", pkt);
				Ok(None)
			}
		}
	}
}
