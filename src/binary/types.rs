#[derive(Debug, Clone)]
pub struct RGB (pub u8, pub u8, pub u8);

#[derive(Debug, Clone)]
pub struct Text (pub TextMode, pub String);

#[derive(Debug, Clone)]
pub struct Vector2 (pub f32, pub f32);

#[derive(Debug, Clone)]
#[repr(u8)]
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
