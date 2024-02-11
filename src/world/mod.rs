use std::fs::File;

use crate::binary::reader::Reader;

#[derive(Debug, Clone)]
pub struct World {

}

#[derive(PartialEq)]
#[repr(u8)]
pub enum FileType {
	None,
	Map,
	World,
	Player,
}

impl World {
	pub fn from_reader(r: &mut Reader) -> World {
		let version = r.read_i32();
		// let (file_type, revision, favorite) = if version >= 135 {
		// 	let ft_and_magic = r.read_u64();
		// 	let magic = ft_and_magic & 
		// } else {
		// 	(FileType::World, 0, false)
		// };
		World {}
	}
}
