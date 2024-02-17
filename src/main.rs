#![allow(clippy::upper_case_acronyms)]

mod binary;
mod network;
mod world;

use binary::reader::Reader;
use network::server::Server;
use std::path::Path;
use world::{binary::SafeReader, types::World};

impl Drop for SafeReader {
	fn drop(&mut self) {
		if self.cur < self.buf.len() {
			println!("dropped SafeReader before EOI ({} bytes remaining)", self.buf.len()-self.cur)
		}
	}
}

impl Drop for Reader<'_> {
	fn drop(&mut self) {
		if self.cur < self.buf.len() {
			println!("dropped Reader before EOI (code: {}, {} bytes remaining)", self.buf[2], self.buf.len()-self.cur)
		}
	}
}

// use directories::UserDirs;

#[tokio::main]
async fn main() {
	// let world = World::from_file(Path::new("/Users/angelolloti/Library/Application Support/Terraria/Worlds/Courtyard_of_Grasshoppers.wld")).unwrap();
	let world = World::from_file(Path::new("C:\\Users\\Dim\\Documents\\My Games\\Terraria\\Worlds\\dim.wld")).unwrap();
	let srv = Server::new(world, "");
	srv.listen("127.0.0.1:7777").await.unwrap();

	// let Some(user_dirs) = UserDirs::new() else {
	// 	panic!("couldn't find user dir")
	// };

	// let Some(doc_dir) = user_dirs.document_dir() else {
	// 	panic!("couldn't find document dir")
	// };

	// let world_dir = doc_dir.join("My Games").join("Terraria").join("Worlds");
	// let world_files = fs::read_dir(world_dir).unwrap();
}
