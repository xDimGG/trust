use std::io::{Seek, Write};

use crate::binary::types::{Text, Vector2, RGB};

pub struct Writer<T> {
	pub dst: T,
}

#[allow(dead_code)]
impl<T: Write> Writer<T> {
	pub fn new(dst: T) -> Self {
		Self { dst }
	}

	pub fn into_inner(self) -> T {
		self.dst
	}

	pub fn write_byte(&mut self, byte: u8) -> Result<(), std::io::Error> {
		self.write_all(&[byte])
	}

	pub fn write_bool(&mut self, b: bool) -> Result<(), std::io::Error> {
		self.write_byte(b as u8)
	}

	pub fn write_i8(&mut self, num: i8) -> Result<(), std::io::Error> {
		self.write_all(&num.to_le_bytes())
	}

	pub fn write_u16(&mut self, num: u16) -> Result<(), std::io::Error> {
		self.write_all(&num.to_le_bytes())
	}

	pub fn write_i16(&mut self, num: i16) -> Result<(), std::io::Error> {
		self.write_all(&num.to_le_bytes())
	}

	pub fn write_u32(&mut self, num: u32) -> Result<(), std::io::Error> {
		self.write_all(&num.to_le_bytes())
	}

	pub fn write_i32(&mut self, num: i32) -> Result<(), std::io::Error> {
		self.write_all(&num.to_le_bytes())
	}

	pub fn write_u64(&mut self, num: u64) -> Result<(), std::io::Error> {
		self.write_all(&num.to_le_bytes())
	}

	pub fn write_i64(&mut self, num: i64) -> Result<(), std::io::Error> {
		self.write_all(&num.to_le_bytes())
	}

	pub fn write_f32(&mut self, num: f32) -> Result<(), std::io::Error> {
		self.write_all(&num.to_le_bytes())
	}

	pub fn write_f64(&mut self, num: f64) -> Result<(), std::io::Error> {
		self.write_all(&num.to_le_bytes())
	}

	pub fn write_length(&mut self, mut len: usize) -> Result<(), std::io::Error> {
		while len >= (1 << 7) {
			self.write_byte((len & 0b1111111) as u8 | (1 << 7))?;
			len >>= 7; // shift the whole thing by seven
		}

		self.write_byte(len as u8)
	}

	pub fn write_string(&mut self, string: impl AsRef<str>) -> Result<(), std::io::Error> {
		let r = string.as_ref();
		self.write_length(r.len())?;
		self.write_all(r.as_bytes())
	}

	pub fn write_text(&mut self, text: Text) -> Result<(), std::io::Error> {
		self.write_byte(text.code())?;
		match text {
			Text::Literal(lit) => {
				self.write_string(lit)?
			},
			Text::Formattable(form, subs) | Text::Key(form, subs) => {
				self.write_string(form)?;
				self.write_byte(subs.len() as u8)?;
				for sub in subs {
					self.write_text(sub)?
				}
			},
			_ => {},
		};
		Ok(())
	}

	pub fn write_rgb(&mut self, rgb: RGB) -> Result<(), std::io::Error> {
		self.write_byte(rgb.0)?;
		self.write_byte(rgb.1)?;
		self.write_byte(rgb.2)
	}

	pub fn write_vector2(&mut self, vec2: Vector2) -> Result<(), std::io::Error> {
		self.write_f32(vec2.0)?;
		self.write_f32(vec2.1)
	}
}

impl<T: Write> Write for Writer<T> {
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
		self.dst.write(buf)
	}

	fn flush(&mut self) -> std::io::Result<()> {
		self.dst.flush()
	}
}

pub struct MessageWriter<T>(Writer<T>);

impl<T: Write + Seek> MessageWriter<T> {
	pub fn new(dst: T, code: u8) -> Result<Self, std::io::Error> {
		let mut w = Writer::new(dst);
		w.write_u16(0)?;
		w.write_byte(code)?;
		Ok(Self(w))
	}

	pub fn inner_mut(&mut self) -> &mut Writer<T> {
		&mut self.0
	}

	pub fn finalize(mut self) -> Result<(), std::io::Error> {
		let pos = self.0.dst.stream_position()?;
		self.0.dst.rewind()?;
		self.0.write_u16(pos as u16)
	}
}
