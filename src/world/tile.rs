use crate::world::types::Format;

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
	pub fn encode(&self, repeat: usize, format: &Format) -> Vec<u8> {
		let mut h_1 = 0;
		let mut h_2 = 0;
		let mut h_3 = 0;
		let mut h_4 = 0;
		let mut buf = vec![0; 4];

		if self.active {
			h_1 |= 2;
			buf.push(self.id as u8);
			if self.id > u8::MAX as i16 {
				buf.push((self.id >> 8) as u8);
				h_1 |= 32;
			}

			if format.importance[self.id as usize] {
				buf.extend(self.frame_x.to_le_bytes());
				buf.extend(self.frame_y.to_le_bytes());
			}

			if self.color > 0 {
				h_3 |= 8;
				buf.push(self.color)
			}
		}

		if self.wall > 0 {
			h_1 |= 4;
			buf.push(self.wall as u8);

			if self.wall_color > 0 {
				h_3 |= 16;
				buf.push(self.wall_color as u8)
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
			buf.push(self.liquid)
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
			buf.push((self.wall >> 8) as u8)
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

		let mut i = 3;
		if h_4 > 0 {
			h_3 |= 1;
			buf[i] = h_4;
			i -= 1;
		}
		if h_3 > 0 {
			h_2 |= 1;
			buf[i] = h_3;
			i -= 1;
		}
		if h_2 > 0 {
			h_1 |= 1;
			buf[i] = h_2;
			i -= 1;
		}

		if repeat > 0 {
			if repeat > u8::MAX as usize {
				h_1 |= 128;
				buf.extend((repeat as u16).to_le_bytes());
			} else {
				h_1 |= 64;
				buf.push(repeat as u8)
			}
		}

		buf[i] = h_1;
		buf[i..].to_vec()
	}
}
