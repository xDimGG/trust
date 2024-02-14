use crate::binary::types::{Text, RGB};

pub struct Reader<'a> {
	buf: &'a [u8],
	cur: usize,
}

#[allow(dead_code)]
impl<'a> Reader<'a> {
	pub fn new(buf: &'a [u8]) -> Self {
		Self { buf, cur: 0 }
	}

	pub fn read_bytes(&mut self, amount: usize) -> &[u8] {
		self.cur += amount;
		&self.buf[(self.cur - amount)..self.cur]
	}

	pub fn read_byte(&mut self) -> u8 {
		self.read_bytes(1)[0]
	}

	pub fn read_bool(&mut self) -> bool {
		self.read_byte() != 0
	}

	pub fn read_i8(&mut self) -> i8 {
		self.read_byte() as i8
	}

	pub fn read_u16(&mut self) -> u16 {
		u16::from_le_bytes(self.read_bytes(2).try_into().unwrap())
	}

	pub fn read_i16(&mut self) -> i16 {
		i16::from_le_bytes(self.read_bytes(2).try_into().unwrap())
	}

	pub fn read_u32(&mut self) -> u32 {
		u32::from_le_bytes(self.read_bytes(4).try_into().unwrap())
	}

	pub fn read_i32(&mut self) -> i32 {
		i32::from_le_bytes(self.read_bytes(4).try_into().unwrap())
	}

	pub fn read_u64(&mut self) -> u64 {
		u64::from_le_bytes(self.read_bytes(8).try_into().unwrap())
	}

	pub fn read_i64(&mut self) -> i64 {
		i64::from_le_bytes(self.read_bytes(8).try_into().unwrap())
	}

	pub fn read_f32(&mut self) -> f32 {
		f32::from_le_bytes(self.read_bytes(4).try_into().unwrap())
	}

	pub fn read_f64(&mut self) -> f64 {
		f64::from_le_bytes(self.read_bytes(8).try_into().unwrap())
	}

	pub fn read_length(&mut self) -> usize {
		let mut length = self.read_byte() as usize;
		let mut shift = 7;
		while length & (1 << shift) != 0 {
			length &= !(1 << shift);
			length |= (self.read_byte() as usize) << shift;
			shift += 7;
		}

		length
	}

	pub fn read_string(&mut self) -> String {
		let length = self.read_length();
		std::str::from_utf8(self.read_bytes(length)).unwrap_or("").to_string()
	}

	pub fn read_text(&mut self) -> Text {
		Text(self.read_byte().into(), self.read_string())
	}

	pub fn read_rgb(&mut self) -> RGB {
		RGB(self.read_byte(), self.read_byte(), self.read_byte())
	}
}
