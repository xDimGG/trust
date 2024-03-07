use std::io::{self, Write};

use crate::world::binary::FileReader;
use crate::world::types::{WorldDecodeError, WALL_COUNT};

use super::transpiled::tile_flags;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Liquid {
	Water,
	Lava,
	Honey,
	Shimmer,
	#[default]
	None,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Tile {
	// pub header: [u8; 4], // remove this later
	pub id: i16, // https://terraria.fandom.com/wiki/Tile_IDs
	pub active: bool,
	pub frame_x: i16,
	pub frame_y: i16,
	pub color: u8,
	pub wall: u16,
	pub wall_color: u16,
	pub liquid: u8,
	pub liquid_kind: Liquid,
	pub wire_1: bool,
	pub wire_2: bool,
	pub wire_3: bool,
	pub wire_4: bool,
	pub actuator: bool,
	pub in_active: bool,
	pub invisible_block: bool,
	pub invisible_wall: bool,
	pub fullbright_block: bool,
	pub fullbright_wall: bool,
	pub half_brick: bool,
	pub slope: u8,
}

impl Tile {
	pub fn decode(r: &mut FileReader) -> Result<(Self, usize), WorldDecodeError> {
		let h_1 = r.read_byte()?;
		let h_2 = if h_1 & 1 == 1 { r.read_byte()? } else { 0 };
		let h_3 = if h_2 & 1 == 1 { r.read_byte()? } else { 0 };
		let h_4 = if h_3 & 1 == 1 { r.read_byte()? } else { 0 };

		let (active, id, frame_x, frame_y, color) = if h_1 & 2 == 2 {
			let id = if h_1 & 32 == 32 {
				r.read_i16()?
			} else {
				r.read_byte()? as i16
			};

			let (x, y) = if tile_flags::FRAME[id as usize] {
				let x = r.read_i16()?;
				let y = r.read_i16()?;
				(x, if id == 144 { 0 } else { y })
			} else {
				(-1, -1)
			};

			let col = if h_3 & 8 == 8 { r.read_byte()? } else { 0 };

			(true, id, x, y, col)
		} else {
			(false, -1, 0, 0, 0)
		};

		let (wall, wall_color) = if h_1 & 4 == 4 {
			(
				r.read_byte()? as u16,
				if h_3 & 16 == 16 {
					r.read_byte()? as u16
				} else {
					0
				},
			)
		} else {
			(0, 0)
		};

		let liquid_bits = (h_1 & 0b11000) >> 3;
		let (liquid_kind, liquid) = if liquid_bits > 0 {
			(
				if h_3 & 128 == 128 {
					Liquid::Shimmer // shimmer
				} else {
					match liquid_bits {
						1 => Liquid::Water,
						2 => Liquid::Lava,
						3 => Liquid::Honey,
						_ => unreachable!(),
					}
				},
				r.read_byte()?,
			)
		} else {
			(Liquid::None, 0)
		};

		let (wire_1, wire_2, wire_3, half_brick, slope) = if h_2 > 1 {
			let n_9 = (h_2 & 0b1110000) >> 4;
			// todo: add check for Main.tileSolid[(int) tile.type] || TileID.Sets.NonSolidSaveSlopes[(int) tile.type])
			let (hb, sl) = if n_9 != 0 {
				(n_9 == 1, n_9 - 1)
			} else {
				(false, 0)
			};
			(h_2 & 2 == 2, h_2 & 4 == 4, h_2 & 8 == 8, hb, sl)
		} else {
			(false, false, false, false, 0)
		};

		let (actuator, in_active, wire_4, wall) = if h_3 > 1 {
			let wall_extended = if h_3 & 64 == 64 {
				let new_wall = (r.read_byte()? as u16) << 8 | wall;
				if new_wall < WALL_COUNT {
					new_wall
				} else {
					0
				}
			} else {
				wall
			};
			(h_3 & 2 == 2, h_3 & 4 == 4, h_3 & 32 == 32, wall_extended)
		} else {
			(false, false, false, wall)
		};

		let (invisible_block, invisible_wall, fullbright_block, fullbright_wall) = if h_4 > 1 {
			(h_4 & 2 == 2, h_4 & 4 == 4, h_4 & 8 == 8, h_4 & 16 == 16)
		} else {
			(false, false, false, false)
		};

		let tile = Tile {
			id,
			active,
			frame_x,
			frame_y,
			color,
			wall,
			wall_color,
			liquid,
			liquid_kind,
			wire_1,
			wire_2,
			wire_3,
			wire_4,
			actuator,
			in_active,
			invisible_block,
			invisible_wall,
			fullbright_block,
			fullbright_wall,
			half_brick,
			slope,
		};

		let repeat = match h_1 >> 6 {
			0 => 0,
			1 => r.read_byte()? as usize,
			_ => r.read_i16()? as usize,
		};

		Ok((tile, repeat))
	}

	pub fn encode(&self, w: &mut impl Write, repeat: usize) -> io::Result<()> {
		let mut h_1 = 0;
		let mut h_2 = 0;
		let mut h_3 = 0;
		let mut h_4 = 0;

		if repeat > 0 {
			if repeat > u8::MAX as usize {
				h_1 |= 128;
			} else {
				h_1 |= 64;
			}
		}

		if self.active {
			h_1 |= 2;
			if self.id > u8::MAX as i16 {
				h_1 |= 32;
			}

			if self.color > 0 {
				h_3 |= 8;
			}
		}

		if self.wall > 0 {
			h_1 |= 4;

			if self.wall_color > 0 {
				h_3 |= 16;
			}
		}

		if self.liquid > 0 {
			let (f_1, f_3) = match self.liquid_kind {
				Liquid::Shimmer => (8, 128),
				Liquid::Lava => (16, 0),
				Liquid::Honey => (24, 0),
				_ => (8, 0),
			};
			h_1 |= f_1;
			h_3 |= f_3;
		}

		if self.wire_1 {
			h_2 |= 2;
		}
		if self.wire_2 {
			h_2 |= 4;
		}
		if self.wire_3 {
			h_2 |= 8;
		}
		if self.half_brick {
			h_2 |= 16;
		} else if self.slope > 0 {
			h_2 |= (self.slope + 1) << 4;
		}
		if self.actuator {
			h_3 |= 2;
		}
		if self.in_active {
			h_3 |= 4;
		}
		if self.wire_4 {
			h_3 |= 32;
		}

		if self.wall > u8::MAX as u16 {
			h_3 |= 64;
		}

		if self.invisible_block {
			h_4 |= 2;
		}
		if self.invisible_wall {
			h_4 |= 4;
		}
		if self.fullbright_block {
			h_4 |= 8;
		}
		if self.fullbright_wall {
			h_4 |= 16;
		}

		if h_4 > 0 {
			h_3 |= 1;
		}
		if h_3 > 0 {
			h_2 |= 1;
		}
		if h_2 > 0 {
			h_1 |= 1;
		}
		w.write_all(&[h_1])?;
		if h_2 > 0 {
			w.write_all(&[h_2])?;
		}
		if h_3 > 0 {
			w.write_all(&[h_3])?;
		}
		if h_4 > 0 {
			w.write_all(&[h_4])?;
		}

		if self.active {
			w.write_all(&[self.id as u8])?;
			if self.id > u8::MAX as i16 {
				w.write_all(&[(self.id >> 8) as u8])?;
			}

			if tile_flags::FRAME[self.id as usize] {
				w.write_all(&self.frame_x.to_le_bytes())?;
				w.write_all(&self.frame_y.to_le_bytes())?;
			}

			if self.color > 0 {
				w.write_all(&[self.color])?;
			}
		}

		if self.wall > 0 {
			w.write_all(&[self.wall as u8])?;

			if self.wall_color > 0 {
				w.write_all(&[self.wall_color as u8])?;
			}
		}

		if self.liquid > 0 {
			w.write_all(&[self.liquid])?;
		}

		if self.wall > u8::MAX as u16 {
			w.write_all(&[(self.wall >> 8) as u8])?;
		}

		if repeat > 0 {
			if repeat > u8::MAX as usize {
				w.write_all(&(repeat as u16).to_le_bytes())?;
			} else {
				w.write_all(&[repeat as u8])?;
			}
		}

		Ok(())
	}
}
