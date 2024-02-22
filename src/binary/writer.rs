use crate::binary::types::{Text, RGB, Vector2};

pub struct Writer {
	pub buf: Vec<u8>,
	message: bool,
}

#[allow(dead_code)]
impl Writer {
	pub fn new() -> Self {
		Self {
			buf: vec![],
			message: false,
		}
	}

	pub fn new_message(code: u8) -> Self {
		Self {
			buf: vec![0, 0, code],
			message: true,
		}
	}

	pub fn finalize(mut self) -> Vec<u8> {
		if self.message {
			let [a, b] = (self.buf.len() as u16).to_le_bytes();
			self.buf[0] = a;
			self.buf[1] = b;
		}
		self.buf
	}

	pub fn write_bytes(&mut self, bytes: Vec<u8>) {
		self.buf.append(&mut bytes.to_vec())
	}

	pub fn write_byte(&mut self, byte: u8) {
		self.buf.push(byte)
	}

	pub fn write_bool(&mut self, b: bool) {
		self.write_byte(b as u8)
	}

	pub fn write_i8(&mut self, num: i8) {
		self.write_bytes(num.to_le_bytes().to_vec())
	}

	pub fn write_u16(&mut self, num: u16) {
		self.write_bytes(num.to_le_bytes().to_vec())
	}

	pub fn write_i16(&mut self, num: i16) {
		self.write_bytes(num.to_le_bytes().to_vec())
	}

	pub fn write_u32(&mut self, num: u32) {
		self.write_bytes(num.to_le_bytes().to_vec())
	}

	pub fn write_i32(&mut self, num: i32) {
		self.write_bytes(num.to_le_bytes().to_vec())
	}

	pub fn write_u64(&mut self, num: u64) {
		self.write_bytes(num.to_le_bytes().to_vec())
	}

	pub fn write_i64(&mut self, num: i64) {
		self.write_bytes(num.to_le_bytes().to_vec())
	}

	pub fn write_f32(&mut self, num: f32) {
		self.write_bytes(num.to_le_bytes().to_vec())
	}

	pub fn write_f64(&mut self, num: f64) {
		self.write_bytes(num.to_le_bytes().to_vec())
	}

	pub fn write_length(&mut self, mut len: usize) {
		while len >= (1 << 7) {
			self.write_byte((len & 0b1111111) as u8 | (1 << 7));
			len >>= 7; // shift the whole thing by seven
		}

		self.write_byte(len as u8)
	}

	pub fn write_string(&mut self, string: String) {
		self.write_length(string.len());
		self.write_bytes(string.as_bytes().to_vec())
	}

	pub fn write_text(&mut self, text: Text) {
		self.write_byte(text.0 as u8);
		self.write_string(text.1)
	}

	pub fn write_rgb(&mut self, rgb: RGB) {
		self.write_byte(rgb.0);
		self.write_byte(rgb.1);
		self.write_byte(rgb.2)
	}

	pub fn write_vector2(&mut self, vec2: Vector2) {
		self.write_f32(vec2.0);
		self.write_f32(vec2.1)
	}
}
