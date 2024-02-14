// utility function for generating u8 bitmask from bools where first param is LSB
pub fn flags(a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool, h: bool) -> u8 {
	a as u8 | (b as u8) << 1 | (c as u8) << 2 | (d as u8) << 3 | (e as u8) << 4 | (f as u8) << 5 | (g as u8) << 6 | (h as u8) << 7
}