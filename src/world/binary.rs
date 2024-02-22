use crate::binary::types::{Text, RGB, Vector2};
use crate::world::types::WorldDecodeError;

pub struct FileReader {
	pub buf: Vec<u8>,
	pub cur: usize,
}

type R<T> = Result<T, WorldDecodeError>;

#[allow(dead_code)]
impl FileReader {
	pub fn new(buf: Vec<u8>) -> Self {
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
		if self.cur <= self.buf.len() {
			Ok(&self.buf[(self.cur - amount)..self.cur])
		} else {
			Err(WorldDecodeError::UnexpectedEOI)
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
		Ok(u16::from_le_bytes(self.read_bytes(2)?.try_into().map_err(|_| WorldDecodeError::InvalidNumber)?))
	}

	pub fn read_i16(&mut self) -> R<i16> {
		Ok(i16::from_le_bytes(self.read_bytes(2)?.try_into().map_err(|_| WorldDecodeError::InvalidNumber)?))
	}

	pub fn read_u32(&mut self) -> R<u32> {
		Ok(u32::from_le_bytes(self.read_bytes(4)?.try_into().map_err(|_| WorldDecodeError::InvalidNumber)?))
	}

	pub fn read_i32(&mut self) -> R<i32> {
		Ok(i32::from_le_bytes(self.read_bytes(4)?.try_into().map_err(|_| WorldDecodeError::InvalidNumber)?))
	}

	pub fn read_u64(&mut self) -> R<u64> {
		Ok(u64::from_le_bytes(self.read_bytes(8)?.try_into().map_err(|_| WorldDecodeError::InvalidNumber)?))
	}

	pub fn read_i64(&mut self) -> R<i64> {
		Ok(i64::from_le_bytes(self.read_bytes(8)?.try_into().map_err(|_| WorldDecodeError::InvalidNumber)?))
	}

	pub fn read_f32(&mut self) -> R<f32> {
		Ok(f32::from_le_bytes(self.read_bytes(4)?.try_into().map_err(|_| WorldDecodeError::InvalidNumber)?))
	}

	pub fn read_f64(&mut self) -> R<f64> {
		Ok(f64::from_le_bytes(self.read_bytes(8)?.try_into().map_err(|_| WorldDecodeError::InvalidNumber)?))
	}

	pub fn read_vector2(&mut self) -> R<Vector2> {
		Ok(Vector2(self.read_f32()?, self.read_f32()?))
	}

	pub fn read_length(&mut self) -> R<usize> {
		let mut length = self.read_byte()? as usize;
		let mut shift = 7;
		while length & (1 << shift) != 0 {
			length &= !(1 << shift);
			length |= (self.read_byte()? as usize) << shift;
			shift += 7;
		}

		Ok(length)
	}

	pub fn read_string(&mut self) -> R<String> {
		let length = self.read_length()?;
		Ok(std::str::from_utf8(self.read_bytes(length)?).map_err(WorldDecodeError::InvalidString)?.to_owned())
	}

	pub fn read_text(&mut self) -> R<Text> {
		Ok(Text(self.read_byte()?.into(), self.read_string()?))
	}

	pub fn read_rgb(&mut self) -> R<RGB> {
		Ok(RGB(self.read_byte()?, self.read_byte()?, self.read_byte()?))
	}
}
