# Converts Main.cs
# First param: Main.cs

import os
import re
import sys
from shared import find_block, rust_header

src = open(os.path.join(os.path.dirname(sys.argv[1]), 'ID', 'TileID.cs'), encoding='utf-8-sig').read()
TILE_COUNT = int(re.search(r'readonly ushort Count = (\d+);', src).group(1))

src = open(sys.argv[1], encoding='utf-8-sig').read().replace('(int) sbyte.MaxValue', '127').replace('(int) byte.MaxValue', '255')

patterns = [
	('FRAME', r'Main.tileFrameImportant\[(\w+)\] = true;|Main.AddEchoFurnitureTile\((\w+)\);'),
	('SOLID', r'Main.tileSolid\[(\w+)\] = true;'),
]
blocks = [
	find_block(src, 'void Initialize_TileAndNPCData1()'),
	find_block(src, 'void Initialize_TileAndNPCData2()'),
]

rust = rust_header(os.path.basename(__file__))

for name, reg in patterns:
	flags = [False] * TILE_COUNT
	for block in blocks:
		for match in re.finditer(reg, block):
			try:
				flags[int(match.group(1) or match.group(2))] = True
			except:
				start, end = re.search(r'for \(int \w+ = (\d+); \w+ <= (\d+)', block[match.start()-150:match.start()]).groups()
				for i in range(int(start), int(end) + 1):
					flags[i] = True

	rust += f'pub const {name}: &[bool] = &[{', '.join(str(b).lower() for b in flags)}];\n'

if len(sys.argv) > 2:
	open(sys.argv[2], 'w').write(rust)
else:
	print(rust)
