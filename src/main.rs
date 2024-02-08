mod server;
mod network;

use std::sync::Arc;

#[tokio::main]
async fn main() {
	let srv = server::Server::new("password").unwrap();
	Arc::new(srv).listen("127.0.0.1:7777").await.unwrap();
}
