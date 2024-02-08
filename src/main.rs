mod server;
mod network;

use server::Server;

#[tokio::main]
async fn main() {
	let srv = Server::new("password");
	srv.listen("127.0.0.1:7777").await.unwrap();
}
