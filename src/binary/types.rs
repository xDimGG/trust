#[derive(Debug, Clone, Default)]
pub struct RGB(pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Default)]
pub struct Vector2(pub f32, pub f32);

#[derive(Debug, Clone, Default)]
pub enum Text {
	Literal(String),
	Formattable(String, Vec<Text>),
	Key(String, Vec<Text>),
	#[default]
	Invalid,
}

impl Text {
	pub fn code(&self) -> u8 {
		match self {
			Text::Literal(_) => 0,
			Text::Formattable(_, _) => 1,
			Text::Key(_, _) => 2,
			_ => 0,
		}
	}
}
