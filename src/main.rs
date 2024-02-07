mod server;
mod messages;

use std::sync::Arc;

#[tokio::main]
async fn main() {
	let srv = server::Server::new("127.0.0.1:7777", "password").await.unwrap();
	Arc::new(srv).start().await.unwrap();
}
