use std::io::{self, Write};

use crate::world::binary::FileReader;
use crate::binary::writer::Writer;
use crate::world::types::WorldDecodeError;

#[derive(Debug, Clone)]
pub struct Entity {
	pub id: i32,
	pub x: i16,
	pub y: i16,
	pub inner: EntityInner,
}

#[derive(Debug, Clone, Default)]
pub struct DisplayDoll {
	pub items: [EntityItem; 8],
	pub dyes: [EntityItem; 8],
}

#[derive(Debug, Clone, Default)]
pub struct HatRack {
	pub items: [EntityItem; 2],
	pub dyes: [EntityItem; 2],
}

#[derive(Debug, Clone, Default)]
pub struct EntityItem {
	pub id: i16,
	pub stack: i16,
	pub prefix: u8,
}

#[derive(Debug, Clone)]
pub enum EntityInner {
	Dummy {
		npc: i16,
	},
	ItemFrame(EntityItem),
	LogicSensor {
		logic_check: u8,
		on: bool,
	},
	DisplayDoll(DisplayDoll),
	WeaponsRack(EntityItem),
	HatRack(HatRack),
	FoodPlatter(EntityItem),
	TeleportationPylon,
}

impl Entity {
	pub fn decode(r: &mut FileReader) -> Result<Self, WorldDecodeError> {
		let kind = r.read_byte()?;
		let id = r.read_i32()?;
		let x = r.read_i16()?;
		let y = r.read_i16()?;

		Ok(Entity {
			id,
			x,
			y,
			inner: EntityInner::decode(r, kind)?,
		})
	}

	pub fn write<T: Write>(&self, w: &mut Writer<T>) -> io::Result<()> {
		w.write_byte(self.inner.kind())?; // reserve first byte for later
		w.write_i32(self.id)?;
		w.write_i16(self.x)?;
		w.write_i16(self.y)?;
		self.inner.write(w)
	}
}

impl EntityInner {
	pub fn kind(&self) -> u8 {
		match &self {
			EntityInner::Dummy { .. } => 0,
			EntityInner::ItemFrame(_) => 1,
			EntityInner::LogicSensor { .. } => 2,
			EntityInner::DisplayDoll(_) => 3,
			EntityInner::WeaponsRack(_) => 4,
			EntityInner::HatRack(_) => 5,
			EntityInner::FoodPlatter(_) => 6,
			EntityInner::TeleportationPylon => 7,
		}
	}

	pub fn write<T: Write>(&self, w: &mut Writer<T>) -> io::Result<()> {
		match self {
			EntityInner::Dummy { npc } => {
				w.write_i16(*npc)
			},
			EntityInner::ItemFrame(frame) => {
				w.write_i16(frame.id)?;
				w.write_byte(frame.prefix)?;
				w.write_i16(frame.stack)
			},
			EntityInner::LogicSensor { logic_check, on } => {
				w.write_byte(*logic_check)?;
				w.write_bool(*on)
			},
			EntityInner::DisplayDoll(doll) => {
				let mut item_flags = 0;
				for (i, item) in doll.items.iter().enumerate().rev() {
					if item.id != 0 {
						item_flags |= 1;
					}
					if i != 0 {
						item_flags <<= 1;
					}
				}
				w.write_byte(item_flags)?;

				let mut dye_flags = 0;
				for (i, dye) in doll.dyes.iter().enumerate().rev() {
					if dye.id != 0 {
						dye_flags |= 1;
					}
					if i != 0 {
						dye_flags <<= 1;
					}
				}
				w.write_byte(dye_flags)?;

				for item in &doll.items {
					if item.id != 0 {
						w.write_i16(item.id)?;
						w.write_byte(item.prefix)?;
						w.write_i16(item.stack)?;
					}
				}

				for dye in &doll.dyes {
					if dye.id != 0 {
						w.write_i16(dye.id)?;
						w.write_byte(dye.prefix)?;
						w.write_i16(dye.stack)?
					}
				}

				Ok(())
			},
			EntityInner::WeaponsRack(rack) => {
				w.write_i16(rack.id)?;
				w.write_byte(rack.prefix)?;
				w.write_i16(rack.stack)
			},
			EntityInner::HatRack(rack) => {
				let mut flags = 0;
				for item in rack.items.iter().rev() {
					if item.id != 0 {
						flags |= 1;
					}
					flags <<= 1;
				}
				for (i, dye) in rack.dyes.iter().enumerate().rev() {
					if dye.id != 0 {
						flags |= 1;
					}
					if i != 0 {
						flags <<= 1;
					}
				}
				w.write_byte(flags)?;

				for item in &rack.items {
					if item.id != 0 {
						w.write_i16(item.id)?;
						w.write_byte(item.prefix)?;
						w.write_i16(item.stack)?;
					}
				}

				for dye in &rack.dyes {
					if dye.id != 0 {
						w.write_i16(dye.id)?;
						w.write_byte(dye.prefix)?;
						w.write_i16(dye.stack)?
					}
				}

				Ok(())
			},
			EntityInner::FoodPlatter(platter) => {
				w.write_i16(platter.id)?;
				w.write_byte(platter.prefix)?;
				w.write_i16(platter.stack)
			},
			EntityInner::TeleportationPylon => Ok(()),
		}
	}

	pub fn decode(r: &mut FileReader, kind: u8) -> Result<Self, WorldDecodeError> {
		match kind {
			0 => Ok(EntityInner::Dummy { npc: r.read_i16()? }),
			1 => Ok(EntityInner::ItemFrame(EntityItem { id: r.read_i16()?, prefix: r.read_byte()?, stack: r.read_i16()? })),
			2 => Ok(EntityInner::LogicSensor { logic_check: r.read_byte()?, on: r.read_bool()? }),
			3 => {
				let mut doll = DisplayDoll::default();
				let mut item_flags = r.read_byte()?;
				let mut dye_flags = r.read_byte()?;

				for item in &mut doll.items {
					if item_flags & 1 == 1 {
						item.id = r.read_i16()?;
						item.prefix = r.read_byte()?;
						item.stack = r.read_i16()?;
					}
					item_flags >>= 1;
				};

				for item in &mut doll.dyes {
					if dye_flags & 1 == 1 {
						item.id = r.read_i16()?;
						item.prefix = r.read_byte()?;
						item.stack = r.read_i16()?;
					}
					dye_flags >>= 1;
				};

				Ok(EntityInner::DisplayDoll(doll))
			}
			4 => Ok(EntityInner::WeaponsRack(EntityItem {
				id: r.read_i16()?,
				prefix: r.read_byte()?,
				stack: r.read_i16()?
			})),
			5 => {
				let mut rack = HatRack::default();
				let mut flags = r.read_byte()?;

				for item in &mut rack.items {
					if flags & 1 == 1 {
						item.id = r.read_i16()?;
						item.prefix = r.read_byte()?;
						item.stack = r.read_i16()?;
					}
					flags >>= 1;
				};

				for item in &mut rack.dyes {
					if flags & 1 == 1 {
						item.id = r.read_i16()?;
						item.prefix = r.read_byte()?;
						item.stack = r.read_i16()?;
					}
					flags >>= 1;
				};

				Ok(EntityInner::HatRack(rack))
			}
			6 => Ok(EntityInner::FoodPlatter(EntityItem {
				id: r.read_i16()?,
				prefix: r.read_byte()?,
				stack: r.read_i16()?,
			})),
			7 => Ok(EntityInner::TeleportationPylon),
			_ => Err(WorldDecodeError::InvalidEntityKind),
		}
	}
}