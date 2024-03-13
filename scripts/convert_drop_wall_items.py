# This script takes a C# files that contains the function KillWall_GetItemDrops and converts its large switch statement to rust code
# Example usage from root directory
# python.exe .\scripts\convert_drop_wall_items.py .\TerrariaServer_Decompiled\Terraria\WorldGen.cs .\src\world\transpiled\drop_wall_items.rs

import os
import re
import sys
from shared import find_block, rust_header

raw = open(sys.argv[1], encoding='utf-8-sig').read()

# remove fluff
src = find_block(raw, 'switch (tileCache.wall)\n      {\n        case 237:').replace('byte.MaxValue', '255')
number = re.compile(r'\d+')
nums = number.finditer(src)
rust = ''

nn = lambda: int(next(nums).group())

for m in nums:
	num = int(m.group())
	i = m.start()
	j = m.end()

	if num == 0:
		continue
	
	if src[j+1] == '&':
		if 'switch' in src[i:i+100]:
			next(nums)
			continue

		rust += f'\n{num}..={nn()} => self.id + {nn()-nn()},'
	elif src[j+1] == '|':
		rust += f'\n{num} | {nn()} => {nn()},'
	elif src[j] in (':', ')'):
		rust += f'\n{num} => {nn()},'

out = rust_header(os.path.basename(__file__))
out += """use crate::world::tile::Tile;

impl Tile {
	pub fn get_dropped_item_wall(&self) -> i16 {
		match self.wall {
			%s
			_ => 0,
		}
	}
}
""" % (rust.strip().replace('\n', '\n\t\t\t'))

if len(sys.argv) > 2:
	open(sys.argv[2], 'w').write(out)
else:
	print(out)
	pass
