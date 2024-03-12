# Converts Main.cs
# First param: Main.cs
# Second param: ID/TileID.cs

# ex: python.exe .\scripts\convert_tile_flags.py .\TerrariaServer_Decompiled\Terraria\Main.cs .\TerrariaServer_Decompiled\Terraria\ID\TileID.cs .\src\world\transpiled\tile_flags.rs

import os
import re
import sys
from shared import find_block, rust_header, pascal_to_camel

src = open(sys.argv[2], encoding='utf-8-sig').read()
src = src.replace('(int) sbyte.MaxValue', '127').replace('(int) byte.MaxValue', '255').replace(' tile', 'Tile')
# convert CreateCustomSet<bool?> to CreateBoolSet
src = src.replace('new bool?(),', '')
src = src.replace(', (object) true', '')
src = src.replace('CreateCustomSet<bool?>', 'CreateBoolSet')
src = src.replace('(ushort) ', '').replace('(object) ', '')
src = src.replace('new int[1]', '1')

COUNT = int(re.search(r'readonly ushort Count = (\d+);', src).group(1))
rust = rust_header(os.path.basename(__file__))

# Snow and Dirt appear twice
seen_snow = False
seen_dirt = False

for name, method, args in re.findall(r'public static \w+\[\] (\w+) = TileID\.Sets\.Factory\.(CreateBoolSet|CreateIntSet)\((.*?)\);', src):
	if name == 'Snow':
		if seen_snow:
			continue
		seen_snow = True

	if name == 'Dirt':
		name = 'ConversionDirt'
		if seen_dirt:
			continue
		seen_dirt = True
	
	args = args.split(', ')
	if args == ['']:
		args = []

	if method == 'CreateBoolSet':
		default = False
		i = 0
		if len(args) > 0:
			if args[0] in ('true', 'false'):
				i = 1
			if args[0] == 'true':
				default = True

		flags = [default] * COUNT
		for n in args[i:]:
			flags[int(n)] = not default
	elif method == 'CreateIntSet':
		flags = [int(args[0])] * COUNT

		for tile, value in zip(args[1::2], args[2::2]):
			flags[int(tile)] = int(value)

	t = 'i32' if method == 'CreateIntSet' else 'bool'
	rust += f'pub const {pascal_to_camel(name)}: &[{t}] = &[{', '.join(str(b).lower() for b in flags)}];\n'


src = open(sys.argv[1], encoding='utf-8-sig').read()
src = src.replace('(int) sbyte.MaxValue', '127').replace('(int) byte.MaxValue', '255')

patterns = [
	('FRAME', r'Main.tileFrameImportant\[(\w+)\] = true;|Main.AddEchoFurnitureTile\((\w+)\);'),
	('SOLID', r'Main.tileSolid\[(\w+)\] = true;'),
	('CONTAINER', r'Main.tileSign\[(\w+)\] = true;'),
	('SIGN', r'Main.tileSign\[(\w+)\] = true;'),
]
blocks = [
	find_block(src, 'void Initialize_TileAndNPCData1()'),
	find_block(src, 'void Initialize_TileAndNPCData2()'),
]

for name, reg in patterns:
	flags = [False] * COUNT
	for block in blocks:
		for match in re.finditer(reg, block):
			try:
				flags[int(match.group(1) or match.group(2))] = True
			except:
				start, end = re.search(r'for \(int \w+ = (\d+); \w+ <= (\d+)', block[match.start()-150:match.start()]).groups()
				for i in range(int(start), int(end) + 1):
					flags[i] = True

	rust += f'pub const {name}: &[bool] = &[{', '.join(str(b).lower() for b in flags)}];\n'

if len(sys.argv) > 3:
	open(sys.argv[3], 'w').write(rust)
else:
	print(rust)
