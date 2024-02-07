#![allow(warnings)]

use macros::message_encoder_decoder;

use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

use std::convert::{TryFrom, TryInto};
use std::str;

#[derive(Debug)]
pub struct RGB (u8, u8, u8);

#[derive(Debug)]
pub enum TextMode {
	Literal,
	Formattable,
	LocalizationKey,
	Invalid,
}

impl From<u8> for TextMode {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::Literal,
			1 => Self::Formattable,
			2 => Self::LocalizationKey,
			_ => Self::Invalid,
		}
	}
}

#[derive(Debug)]
pub struct Text (pub TextMode, pub String);

#[message_encoder_decoder]
pub enum Message<'a> {
	/// $01 ->
	VersionIdentifier(String),
	/// $02 <-
	ConnectionRefuse(Text),
	/// $03 <-
	ConnectionApprove {
		pub who_am_i: u8,
		pub always_false: bool,
	},
	/// $04 ->
	PlayerAppearance {
		pub id: u8,
		pub skin_variant: u8,
		pub hair: u8,
		pub name: String,
		pub hair_dye: u8,
		pub hide_accessory: u16,
		pub hide_misc: u8,
		pub hair_color: RGB,
		pub skin_color: RGB,
		pub eye_color: RGB,
		pub shirt_color: RGB,
		pub undershirt_color: RGB,
		pub pants_color: RGB,
		pub shoe_color: RGB,
		pub difficulty: u8,
		pub extra_accessory: u8,
	},
	/// $05 ->
	PlayerInventorySlot {
		pub client_id: u8,
		pub slot_id: u16,
		pub amount: u16,
		pub prefix: u8,
		pub item_id: u16,
	},
	/// $06 ->
	WorldRequest,
	/// $08 ->
	SpawnRequest {
		pub x: i32,
		pub y: i32,
	},
	/// $10 ->
	PlayerHealth {
		pub client_id: u8,
		pub current: u16,
		pub maximum: u16,
	},
	/// $25 <-
	PasswordRequest,
	/// $26 ->
	PasswordResponse(String),
	/// $2A ->
	PlayerMana {
		pub client_id: u8,
		pub current: u16,
		pub maximum: u16,
	},
	/// $32 ->
	PlayerBuffs {
		pub client_id: u8,
		pub buffs: [u16; 22],
	},
	/// $44 ->
	UUID(String),
	/// $53 <-
	KillCount {
		pub id: u16,
		pub amount: u32,
	},
	/// $65 <-
	PillarsStatus {
		pub solar: u16,
		pub vortex: u16,
		pub nebula: u16,
		pub stardust: u16,
	},
	/// $00 <->
	Unknown(u8, &'a [u8]),
}

struct MessageReader<'a> {
	buf: &'a [u8],
	cur: usize,
}

impl<'a> MessageReader<'a> {
	fn new(buf: &'a [u8], cur: usize) -> Self {
		Self { buf, cur }
	}

	fn read_bytes(&mut self, amount: usize) -> &[u8] {
		self.cur += amount;
		&self.buf[(self.cur - amount)..self.cur]
	}

	fn read_byte(&mut self) -> u8 {
		self.read_bytes(1)[0]
	}

	fn read_bool(&mut self) -> bool {
		self.read_byte() != 0
	}

	fn read_i8(&mut self) -> i8 {
		self.read_byte() as i8
	}

	fn read_u16(&mut self) -> u16 {
		u16::from_le_bytes(self.read_bytes(2).try_into().unwrap())
	}

	fn read_i16(&mut self) -> i16 {
		i16::from_le_bytes(self.read_bytes(2).try_into().unwrap())
	}

	fn read_u32(&mut self) -> u32 {
		u32::from_le_bytes(self.read_bytes(4).try_into().unwrap())
	}

	fn read_i32(&mut self) -> i32 {
		i32::from_le_bytes(self.read_bytes(4).try_into().unwrap())
	}

	fn read_string(&mut self) -> String {
		let length = self.read_byte() as usize;
		str::from_utf8(self.read_bytes(length)).unwrap_or("").to_string()
	}

	fn read_text(&mut self) -> Text {
		Text(self.read_byte().into(), self.read_string())
	}

	fn read_rgb(&mut self) -> RGB {
		RGB(self.read_byte(), self.read_byte(), self.read_byte())
	}
}

impl<'a> Message<'a> {
	pub async fn write(self, stream: &mut TcpStream) -> Result<usize, &str> {
		let buffer: Vec<u8> = self.try_into()?;
		stream.write(&buffer).await.map_err(|_| "Error while writing")
	}
}

struct MessageWriter {
	buf: Vec<u8>,
}

impl<'a> MessageWriter {
	fn new(code: u8) -> Self {
		Self { buf: vec![0, 0, code] }
	}

	fn finalize(mut self) -> Vec<u8> {
		let [a, b] = (self.buf.len() as u16).to_le_bytes();
		self.buf[0] = a;
		self.buf[1] = b;
		self.buf
	}

	fn write_bytes(mut self, bytes: &[u8]) -> Self {
		self.buf.append(&mut bytes.to_vec());
		self
	}

	#[allow(dead_code)]
	fn write_byte(mut self, byte: u8) -> Self {
		self.buf.push(byte);
		self
	}

	fn write_bool(self, b: bool) -> Self {
		self.write_byte(b as u8)
	}

	fn write_i8(self, num: i8) -> Self {
		self.write_bytes(&mut num.to_le_bytes().to_vec())
	}

	fn write_u16(self, num: u16) -> Self {
		self.write_bytes(&mut num.to_le_bytes().to_vec())
	}

	fn write_i16(self, num: i16) -> Self {
		self.write_bytes(&mut num.to_le_bytes().to_vec())
	}

	fn write_u32(self, num: u32) -> Self {
		self.write_bytes(&mut num.to_le_bytes().to_vec())
	}

	fn write_i32(self, num: i32) -> Self {
		self.write_bytes(&mut num.to_le_bytes().to_vec())
	}

	fn write_string(mut self, string: String) -> Self {
		self
			.write_byte(string.len() as u8)
			.write_bytes(string.as_bytes())
			.write_byte(0)
	}

	fn write_text(mut self, text: Text) -> Self {
		self
			.write_byte(text.0 as u8)
			.write_string(text.1)
	}

	fn write_rgb(mut self, rgb: RGB) -> Self {
		self
			.write_byte(rgb.0)
			.write_byte(rgb.1)
			.write_byte(rgb.2)
	}
}
