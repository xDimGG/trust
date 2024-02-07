use tokio::net::{TcpListener, TcpStream};
use tokio::io::{self, AsyncReadExt};
use tokio::sync::RwLock;

use std::sync::Arc;

use crate::messages::{Message, Text, TextMode, ConnectionApprove};

const GAME_VERSION: &'static str = "Terraria279";

pub struct Server {
	// clients: 
	password: RwLock<String>,
	listener: TcpListener,
}
impl Server {
	pub async fn new(address: &str, password: &str) -> io::Result<Server> {
		let listener = TcpListener::bind(address).await?;

		Ok(Server {
			password: RwLock::new(password.to_owned()),
			listener,
		})
	}

	pub async fn start(self: Arc<Self>) -> io::Result<()> {
		loop {
			let srv = Arc::clone(&self);
			let (stream, _) = srv.listener.accept().await?;
			tokio::spawn(async move {
				srv.accept(stream).await
			});
		}
	}

	async fn accept(&self, mut stream: TcpStream) -> io::Result<()> {
		loop {
			let mut length = [0u8; 2];
			stream.read(&mut length).await?;
			let length = u16::from_le_bytes(length);

			if length < 2 {
				continue;
			}

			let mut buffer = vec![0u8; length as usize - 2];
			stream.read(&mut buffer).await?;

			dbg!(Message::from(buffer.as_slice()));

			match Message::from(buffer.as_slice()) {
				Message::VersionIdentifier(version) => {
					if version == GAME_VERSION {
						let password = self.password.read().await;
						if password.is_empty() {
							let msg = Message::ConnectionApprove(ConnectionApprove {
								who_am_i: 0,
								always_false: false,
							});
							msg.write(&mut stream).await.unwrap();
						} else {
							Message::PasswordRequest.write(&mut stream).await.unwrap();
						}
					} else {
						println!("Player tried joining with version {}", version);
						Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.4".to_owned())).write(&mut stream).await.unwrap();
					}
				}
			// 	Message::PlayerAppearance(pa) => { dbg!(pa); },
			// 	Message::PlayerInventorySlot(pis) => { dbg!(pis); },
			// 	Message::WorldRequest => {
			// 		Message::Unknown(0x07, b"\xb2\x6a\x00\x00\x00\x00\x68\x10\xb0\x04\x33\x08\xef\x00\x50\x01\xb0\x01\x37\xd4\x43\x51\x05trust\x03\xad\x39\xad\x7f\x7e\x13\x3f\x46\x9f\x72\x8d\xcc\xca\x4c\xc0\xd7\x01\x00\x00\x00\xe4\x00\x00\x00\x06\x07\x0a\x08\x01\x01\x05\x05\x01\x05\x03\x04\x02\x00\x02\x01\x01\x13\x83\x40\xbd\x00\x94\x06\x00\x00\x68\x10\x00\x00\x68\x10\x00\x00\x00\x02\x00\x00\x34\x04\x00\x00\x68\x10\x00\x00\x68\x10\x00\x00\x02\x05\x03\x07\x00\x02\x00\x00\x01\x05\x05\x01\x05\x03\x04\x02\x00\x00\x00\x00\x00\x02\x00\x00\x00\x00\x00\x00\x07\x00\xa7\x00\x09\x00\x08\x00\xff\xff\xff\xff\xff\xff\x00\x00\x00\x00\x00\x00\x00\x00\x00\x46\xe7\x19\x3e").write(&mut stream).await.unwrap();
			// 	},
				Message::PasswordResponse(pass) => {
					let password = self.password.read().await;
					if pass == password.as_str() {
						let msg = Message::ConnectionApprove(ConnectionApprove {
							who_am_i: 0,
							always_false: false,
						});
						msg.write(&mut stream).await.unwrap();
					} else {
						Message::ConnectionRefuse(Text(TextMode::LocalizationKey, "LegacyMultiplayer.1".to_owned())).write(&mut stream).await.unwrap();
					}
				},
			// 	Message::PlayerHealth(ph) => { dbg!(ph); },
			// 	Message::UUID(uuid) => println!("Got UUID: {}", uuid),
			// 	Message::PlayerMana(pm) => { dbg!(pm); },
			// 	Message::PlayerBuffs(pb) => { dbg!(pb); },
			// 	Message::Unknown(code, buf) => println!("Unknown ({:#x}): {:?}", code, buf),
				_ => println!("Not yet implemented packet: {:?}", buffer),
			}
		}
	}
}
