use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt, AsyncWrite};
use tokio::sync::{RwLock, mpsc};
use std::pin::Pin;
use std::sync::Arc;

use crate::network::messages::{self, Message, Text, TextMode, ConnectionApprove};

const GAME_VERSION: &'static str = "Terraria279";
const MAX_CLIENTS: usize = 256;

pub struct Client<'a> {
	pub details: messages::PlayerDetails,
	pub channel: mpsc::UnboundedReceiver<Message<'a>>,
}

pub struct Server<'a> {
	password: RwLock<String>,
	clients: RwLock<[Option<Client<'a>>; MAX_CLIENTS]>,
}

impl<'a> Server<'a> {
	pub fn new(password: &str) -> io::Result<Server> {
		// https://github.com/rust-lang/rust/issues/44796#issuecomment-967747810
		const INIT_CLIENT_NONE: Option<Client> = None;

		Ok(Server {
			password: RwLock::new(password.to_owned()),
			clients: RwLock::new([INIT_CLIENT_NONE; MAX_CLIENTS]),
		})
	}

	pub async fn listen(self: Arc<&'a Self>, address: &str) -> io::Result<()> {
		let listener = TcpListener::bind(address).await?;

		loop {
			let (stream, _) = listener.accept().await?;
			let rc = Arc::clone(&self);
			tokio::spawn(async move {
				rc.accept(stream).await
			});
		}
	}

	async fn accept(&self, mut stream: TcpStream) -> io::Result<()> {
		let (mut rh, mut wh) = stream.split();
		let (tx, mut rx) = mpsc::unbounded_channel();

		loop {
			let mut length = [0u8; 2];
			rh.read(&mut length).await?;
			let length = u16::from_le_bytes(length);

			if length < 2 {
				continue;
			}

			let mut buffer = vec![0u8; length as usize - 2];
			rh.read(&mut buffer).await?;

			match Message::from(buffer.as_slice()) {
				Message::VersionIdentifier(version) => {
					if version == GAME_VERSION {
						let password = self.password.read().await;
						if password.is_empty() {
							let msg = Message::ConnectionApprove(ConnectionApprove {
								id: 0,
								b: false,
							});
							msg.write(Pin::new(&mut wh)).await.unwrap();
						} else {
							Message::PasswordRequest.write(Pin::new(&mut wh)).await.unwrap();
						}
					} else {
						println!("Player tried joining with version {}", version);
						Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.4".to_owned())).write(Pin::new(&mut wh)).await.unwrap();
					}
				}
				Message::PlayerDetails(pd) => {
					let mut clients = self.clients.write().await;
					for i in 0..MAX_CLIENTS {
						if clients[i].is_none() {
							clients[i] = Some(Client {
								details: pd,
								writer: wh.clone(),
							});
							break
						}
					}
				},
			// 	Message::PlayerInventorySlot(pis) => { dbg!(pis); },
			// 	Message::WorldRequest => {
			// 		Message::Unknown(0x07, b"\xb2\x6a\x00\x00\x00\x00\x68\x10\xb0\x04\x33\x08\xef\x00\x50\x01\xb0\x01\x37\xd4\x43\x51\x05trust\x03\xad\x39\xad\x7f\x7e\x13\x3f\x46\x9f\x72\x8d\xcc\xca\x4c\xc0\xd7\x01\x00\x00\x00\xe4\x00\x00\x00\x06\x07\x0a\x08\x01\x01\x05\x05\x01\x05\x03\x04\x02\x00\x02\x01\x01\x13\x83\x40\xbd\x00\x94\x06\x00\x00\x68\x10\x00\x00\x68\x10\x00\x00\x00\x02\x00\x00\x34\x04\x00\x00\x68\x10\x00\x00\x68\x10\x00\x00\x02\x05\x03\x07\x00\x02\x00\x00\x01\x05\x05\x01\x05\x03\x04\x02\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x07\x00\xa7\x00\x09\x00\x08\x00\xff\xff\xff\xff\xff\xff\x00\x00\x00\x00\x00\x00\x00\x00\x00\x46\xe7\x19\x3e").write(&mut stream).await.unwrap();
			// 	},
				Message::PasswordResponse(pass) => {
					let password = self.password.read().await;
					if pass == password.as_str() {
						let msg = Message::ConnectionApprove(ConnectionApprove {
							id: 0,
							b: false,
						});
						msg.write(Pin::new(&mut wh)).await.unwrap();
					} else {
						Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.1".to_owned())).write(Pin::new(&mut wh)).await.unwrap();
					}
				},
			// 	Message::PlayerHealth(ph) => { dbg!(ph); },
				// Message::UUID(uuid) => println!("Got UUID: {}", uuid),
			// 	Message::PlayerMana(pm) => { dbg!(pm); },
			// 	Message::PlayerBuffs(pb) => { dbg!(pb); },
				Message::Unknown(code, buf) => println!("Unknown ({:#x}): {:?}", code, buf),
				pkt => println!("Not yet implemented packet: {:?}", pkt),
			}
		}
	}
}
