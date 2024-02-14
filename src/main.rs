#![allow(clippy::upper_case_acronyms)]

mod binary;
mod network;
mod world;

use network::server::Server;
use directories::UserDirs;

use std::{fs, path::Path};

use world::World;

#[tokio::main]
async fn main() {
	// let srv = Server::new("password");
	// srv.listen("127.0.0.1:7777").await.unwrap();

	// let Some(user_dirs) = UserDirs::new() else {
	// 	panic!("couldn't find user dir")
	// };

	// let Some(doc_dir) = user_dirs.document_dir() else {
	// 	panic!("couldn't find document dir")
	// };

	// let world_dir = doc_dir.join("My Games").join("Terraria").join("Worlds");
	// let world_files = fs::read_dir(world_dir).unwrap();

	match World::from_file(Path::new("C:\\Users\\Dim\\Documents\\My Games\\Terraria\\Worlds\\college_is_easy.wld")) {
		Ok(w) => { dbg!(&w.entities); }
		Err(e) => { println!("Parse Error: {}", e); }
	};
}
