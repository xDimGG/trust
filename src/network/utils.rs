#[allow(clippy::too_many_arguments)]
// utility function for generating u8 bitmask from bools where first param is LSB
pub fn flags(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool, h: bool) -> u8 {
	a as u8 | (b as u8) << 1 | (c as u8) << 2 | (d as u8) << 3 | (e as u8) << 4 | (f as u8) << 5 | (g as u8) << 6 | (h as u8) << 7
}

pub fn get_section_x(x: i32) -> i32 {
	x / 200
}

pub fn get_section_y(y: i32) -> i32 {
	y / 150
}

pub fn get_tile_x_start(sec_x: i32) -> i32 {
	sec_x * 200
}

pub fn get_tile_x_end(sec_x: i32) -> i32 {
	(sec_x + 1) * 200
}

pub fn get_tile_y_start(sec_y: i32) -> i32 {
	sec_y * 150
}

pub fn get_tile_y_end(sec_y: i32) -> i32 {
	(sec_y + 1) * 150
}
