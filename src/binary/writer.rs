use crate::binary::types::{Text, RGB};

pub struct Writer {
	buf: Vec<u8>,
}

#[allow(dead_code)]
impl Writer {
	pub fn new(code: u8) -> Self {
		Self { buf: vec![0, 0, code] }
	}

	pub fn finalize(mut self) -> Vec<u8> {
		let [a, b] = (self.buf.len() as u16).to_le_bytes();
		self.buf[0] = a;
		self.buf[1] = b;
		self.buf
	}

	pub fn write_bytes(mut self, bytes: &[u8]) -> Self {
		self.buf.append(&mut bytes.to_vec());
		self
	}

	pub fn write_byte(mut self, byte: u8) -> Self {
		self.buf.push(byte);
		self
	}

	pub fn write_bool(self, b: bool) -> Self {
		self.write_byte(b as u8)
	}

	pub fn write_i8(self, num: i8) -> Self {
		self.write_bytes(&num.to_le_bytes())
	}

	pub fn write_u16(self, num: u16) -> Self {
		self.write_bytes(&num.to_le_bytes())
	}

	pub fn write_i16(self, num: i16) -> Self {
		self.write_bytes(&num.to_le_bytes())
	}

	pub fn write_u32(self, num: u32) -> Self {
		self.write_bytes(&num.to_le_bytes())
	}

	pub fn write_i32(self, num: i32) -> Self {
		self.write_bytes(&num.to_le_bytes())
	}

	pub fn write_u64(self, num: u64) -> Self {
		self.write_bytes(&num.to_le_bytes())
	}

	pub fn write_i64(self, num: i64) -> Self {
		self.write_bytes(&num.to_le_bytes())
	}

	pub fn write_string(self, string: String) -> Self {
		self
			.write_byte(string.len() as u8)
			.write_bytes(string.as_bytes())
			.write_byte(0)
	}

	pub fn write_text(self, text: Text) -> Self {
		self
			.write_byte(text.0 as u8)
			.write_string(text.1)
	}

	pub fn write_rgb(self, rgb: RGB) -> Self {
		self
			.write_byte(rgb.0)
			.write_byte(rgb.1)
			.write_byte(rgb.2)
	}
}
