#![allow(clippy::upper_case_acronyms)]

mod binary;
mod network;
mod world;

use binary::reader::Reader;
use network::server::Server;
use std::{path::Path, time::{Duration, UNIX_EPOCH}};
use world::{binary::FileReader, types::World};

impl Drop for FileReader {
	fn drop(&mut self) {
		if self.cur < self.buf.len() {
			println!("dropped FileReader before EOI ({} bytes remaining)", self.buf.len()-self.cur)
		}
	}
}

impl Drop for Reader<'_> {
	fn drop(&mut self) {
		if self.cur < self.buf.len() {
			println!("dropped Reader before EOI (code: {}, {} bytes remaining)", self.buf[0], self.buf.len()-self.cur)
		}
	}
}

#[tokio::main]
async fn main() {
	// let world = World::from_file(Path::new("C:\\Users\\Dim\\Documents\\My Games\\Terraria\\Worlds\\dim.wld")).unwrap();
	let world = World::from_file(Path::new("/mnt/c/Users/Dim/Documents/My Games/Terraria/Worlds/dim.wld")).unwrap();
	// let world = World::from_file(Path::new("/Users/angelolloti/Library/Application Support/Terraria/Worlds/workshop.wld")).unwrap();
	let srv = Server::new(world, "pass");
	srv.listen("127.0.0.1:7778").await.unwrap();

	// let Some(user_dirs) = UserDirs::new() else {
	// 	panic!("couldn't find user dir")
	// };

	// let Some(doc_dir) = user_dirs.document_dir() else {
	// 	panic!("couldn't find document dir")
	// };

	// let world_dir = doc_dir.join("My Games").join("Terraria").join("Worlds");
	// let world_files = fs::read_dir(world_dir).unwrap();

}
