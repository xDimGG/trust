use crate::binary::types::{Text, RGB};
use crate::world::WorldParseError;

pub struct SafeReader<'a> {
	buf: &'a [u8],
	cur: usize,
}

type R<T> = Result<T, WorldParseError>;

#[allow(dead_code)]
impl<'a> SafeReader<'a> {
	pub fn new(buf: &'a [u8]) -> Self {
		Self { buf, cur: 0 }
	}

	pub fn seek(&mut self, index: usize) {
		self.cur = index;
	}

	pub fn skip(&mut self, delta: usize) {
		self.cur += delta;
	}

	pub fn get_cur(&self) -> usize {
		self.cur
	}

	pub fn read_bytes(&mut self, amount: usize) -> R<&[u8]> {
		self.cur += amount;
		if self.cur < self.buf.len() {
			Ok(&self.buf[(self.cur - amount)..self.cur])
		} else {
			Err(WorldParseError::UnexpectedEOF)
		}
	}

	pub fn read_byte(&mut self) -> R<u8> {
		Ok(self.read_bytes(1)?[0])
	}

	pub fn read_bool(&mut self) -> R<bool> {
		Ok(self.read_byte()? != 0)
	}

	pub fn read_i8(&mut self) -> R<i8> {
		Ok(self.read_byte()? as i8)
	}

	pub fn read_u16(&mut self) -> R<u16> {
		Ok(u16::from_le_bytes(self.read_bytes(2)?.try_into().map_err(|_| WorldParseError::InvalidNumber)?))
	}

	pub fn read_i16(&mut self) -> R<i16> {
		Ok(i16::from_le_bytes(self.read_bytes(2)?.try_into().map_err(|_| WorldParseError::InvalidNumber)?))
	}

	pub fn read_u32(&mut self) -> R<u32> {
		Ok(u32::from_le_bytes(self.read_bytes(4)?.try_into().map_err(|_| WorldParseError::InvalidNumber)?))
	}

	pub fn read_i32(&mut self) -> R<i32> {
		Ok(i32::from_le_bytes(self.read_bytes(4)?.try_into().map_err(|_| WorldParseError::InvalidNumber)?))
	}

	pub fn read_u64(&mut self) -> R<u64> {
		Ok(u64::from_le_bytes(self.read_bytes(8)?.try_into().map_err(|_| WorldParseError::InvalidNumber)?))
	}

	pub fn read_i64(&mut self) -> R<i64> {
		Ok(i64::from_le_bytes(self.read_bytes(8)?.try_into().map_err(|_| WorldParseError::InvalidNumber)?))
	}

	pub fn read_f32(&mut self) -> R<f32> {
		Ok(f32::from_le_bytes(self.read_bytes(4)?.try_into().map_err(|_| WorldParseError::InvalidNumber)?))
	}

	pub fn read_f64(&mut self) -> R<f64> {
		Ok(f64::from_le_bytes(self.read_bytes(8)?.try_into().map_err(|_| WorldParseError::InvalidNumber)?))
	}

	pub fn read_string(&mut self) -> R<String> {
		let length = self.read_byte()? as usize;
		match std::str::from_utf8(self.read_bytes(length)?) {
			Ok(c) => Ok(c.to_string()),
			Err(_) => Err(WorldParseError::InvalidString),
		}
	}

	pub fn read_text(&mut self) -> R<Text> {
		Ok(Text(self.read_byte()?.into(), self.read_string()?))
	}

	pub fn read_rgb(&mut self) -> R<RGB> {
		Ok(RGB(self.read_byte()?, self.read_byte()?, self.read_byte()?))
	}
}
