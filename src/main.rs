#![allow(clippy::upper_case_acronyms)]

mod binary;
mod network;
mod world;

use binary::reader::Reader;
use network::server::Server;
use std::{num::ParseIntError, path::Path};
use world::{binary::SafeReader, types::{Liquid, World, WALL_COUNT}};

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
			println!("dropped Reader before EOI (code: {}, {} bytes remaining)", self.buf[0], self.buf.len()-self.cur)
		}
	}
}

// use directories::UserDirs;

fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
	(0..s.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&s[i..i + 2], 16))
		.collect()
}

#[tokio::main]
async fn main() {
	let world = World::from_file(Path::new("/Users/angelolloti/Library/Application Support/Terraria/Worlds/Courtyard_of_Grasshoppers.wld")).unwrap();
	let f = decode_hex("280a000096000000c8009600403042c40446c4490146bd495446c4490146bd490c02bd405d02c4000350c402c406c44946bd492506c44906bd4946c4490406bd4906c44946bd492346c4490446bd490b02bd40600350c442bd0246bd490842bd0846bd491146c4490546bd492546c4490546bd490b42bd01406242bd0246bd490442bd02400602bd46bd491146c4490446bd492546c4490646bd490842bd020340bd406302bd0042bd050340bd0002bd40040310bd42bd0146bd491046c4490546bd492142bd0402c446c4490546bd490142bd050340bd0002bd407342bd0146bd491306c44906bd4906c44942c40142bd0246bd490806c44906bd4946c44904044946bd490d02bd400342c40306c44942c4030340c4407b0350bd02bd46bd491602bd400242bd0346bd490446c4490846bd490b42bd010340bd40060350c402c40340c40002c40340c4407d0350bd42bd0346bd491202bd40050350bd02bd46bd490446c4490806bd4942bd0246bd490442bd0240900350bd42bd0246bd490f42bd01400542bd0246bd490246c4490642c40202bd000350bd42bd040340bd0002bd409342bd100340bd400602bd000350bd42bd0442c4040340c440ab02bd80036d000000000000").unwrap();
	let mut r = Reader::new(f.as_slice());
	let x_start = r.read_i32() as usize;
	let y_start = r.read_i32() as usize;
	let width = r.read_i16() as usize;
	let height = r.read_i16() as usize;

	// dbg!(world.format.importance);

	let mut repeat = 0;

	for x in x_start..(x_start + width) {
		for y in y_start..(y_start + height) {
			if repeat > 0 {
				repeat -= 1;
				continue;
			}

			// Header bytes
			let h_1 = r.read_byte();
			let h_2 = if h_1 & 1 == 1 { r.read_byte() } else { 0 };
			let h_3 = if h_2 & 1 == 1 { r.read_byte() } else { 0 };
			let h_4 = if h_3 & 1 == 1 { r.read_byte() } else { 0 };

			let (active, id, frame_x, frame_y, color) = if h_1 & 2 == 2 {
				let id = if h_1 & 32 == 32 {
					r.read_i16()
				} else {
					r.read_byte() as i16
				};

				let (x, y) = if world.format.importance[id as usize] {
					let x = r.read_i16();
					let y = r.read_i16();
					(x, if id == 144 { 0 } else { y })
				} else { (-1, -1) };

				let col = if h_3 & 8 == 8 { r.read_byte() } else { 0 };

				(true, id, x, y, col)
			} else {
				(false, -1, 0, 0, 0)
			};

			let (wall, wall_color) = if h_1 & 4 == 4 {
				(r.read_byte() as u16, if h_3 & 16 == 16 { r.read_byte() as u16 } else { 0 })
			} else { (0, 0) };

			let liquid_bits = (h_1 & 0b11000) >> 3;
			let (liquid, liquid_header) = if liquid_bits != 0 {
				(if h_3 & 128 == 128 {
					Liquid::Shimmer // shimmer
				} else {
					match liquid_bits {
						2 => Liquid::Lava, // lava
						3 => Liquid::Honey, // honey
						_ => Liquid::Water, // water
					}
				}, r.read_byte())
			} else {
				(Liquid::None, 0)
			};

			let (wire_1, wire_2, wire_3, half_brick, slope) = if h_2 > 1 {
				let n_9 = (h_2 & 0b1110000) >> 4;
				// todo: add check for Main.tileSolid[(int) tile.type] || TileID.Sets.NonSolidSaveSlopes[(int) tile.type])
				let (hb, sl) = if n_9 != 0 {
					(n_9 == 1, n_9 - 1)
				} else { (false, 0) };
				(h_2 & 2 == 2, h_2 & 4 == 4, h_2 & 8 == 8, hb, sl)
			} else { (false, false, false, false, 0) };

			let (actuator, in_active, wire_4, wall) = if h_3 > 1 {
				let wall_extended = if h_3 & 64 == 64 {
					let new_wall = (r.read_byte() as u16) << 8 | wall;
					if new_wall < WALL_COUNT {
						new_wall
					} else {
						0
					}
				} else { wall };
				(h_3 & 2 == 2, h_3 & 4 == 4, h_3 & 32 == 32, wall_extended)
			} else { (false, false, false, wall) };

			let (invisible_block, invisible_wall, fullbright_block, fullbright_wall) = if h_4 > 1 {
				(h_4 & 2 == 2, h_4 & 4 == 4, h_4 & 8 == 8, h_4 & 16 == 16)
			} else { (false, false, false, false) };

			repeat = match (h_1 & 0b11000000) >> 6 {
				0 => 0,
				1 => r.read_byte() as i32,
				_ => r.read_i16() as i32,
			};

			println!("tile id {} repeats {}", id, repeat)
		}
	}

	// let world = World::from_file(Path::new("/Users/angelolloti/Library/Application Support/Terraria/Worlds/Courtyard_of_Grasshoppers.wld")).unwrap();
	// let world = World::from_file(Path::new("C:\\Users\\Dim\\Documents\\My Games\\Terraria\\Worlds\\dim.wld")).unwrap();
	// let srv = Server::new(world, "");
	// srv.listen("127.0.0.1:7777").await.unwrap();

	// let Some(user_dirs) = UserDirs::new() else {
	// 	panic!("couldn't find user dir")
	// };

	// let Some(doc_dir) = user_dirs.document_dir() else {
	// 	panic!("couldn't find document dir")
	// };

	// let world_dir = doc_dir.join("My Games").join("Terraria").join("Worlds");
	// let world_files = fs::read_dir(world_dir).unwrap();
}
