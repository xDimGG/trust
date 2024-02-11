mod binary;
mod network;
mod world;

use binary::reader::Reader;
use network::server::Server;
use directories::UserDirs;

use std::fs;

use world::World;

#[tokio::main]
async fn main() {
	let srv = Server::new("password");
	srv.listen("127.0.0.1:7777").await.unwrap();

	// let Some(user_dirs) = UserDirs::new() else {
	// 	panic!("couldn't find user dir")
	// };

	// let Some(doc_dir) = user_dirs.document_dir() else {
	// 	panic!("couldn't find document dir")
	// };

	// let world_dir = doc_dir.join("My Games").join("Terraria").join("Worlds");
	// let world_files = fs::read_dir(world_dir).unwrap();

	// for entry in world_files {
	// 	let path = entry.unwrap().path();
	// 	let stream = fs::read(path).unwrap();
	// 	let mut reader = Reader::new(stream.as_slice(), 0);
	// 	dbg!(World::from_reader(&mut reader));
	// }

	// fs::read_dir("path")
}
