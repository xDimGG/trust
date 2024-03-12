# First param: ID/ItemID.cs

# ex: python.exe .\scripts\convert_item_flags.py .\TerrariaServer_Decompiled\Terraria\ID\ItemID.cs .\src\world\transpiled\item_flags.rs

# note: does not cover ALL constants. some are annoying to do
# if i need them later, i'll do them, but i dont think i'll ever need them

import os
import re
import sys
from shared import rust_header, pascal_to_camel

src = open(sys.argv[1], encoding='utf-8-sig').read()
src = src.replace('(int) sbyte.MaxValue', '127').replace('(int) byte.MaxValue', '255')
# convert CreateCustomSet<bool?> to CreateBoolSet
src = src.replace('CreateCustomSet<bool?>(new bool?(), ', 'CreateBoolSet(').replace('?', '')
src = re.sub(r'\(object\) \(short\) \d+, \(object\) false, ', '', src)
src = src.replace(', (object) true', '')
src = src.replace('(short) ', '').replace('(ushort) ', '').replace('(object) ', '')
src = src.replace('new int[1]', '1')

KILLS_TO_BANNER = re.search(r'public static int DefaultKillsForBannerNeeded = (\d+);', src).group(1)
src = src.replace('ItemID.Sets.DefaultKillsForBannerNeeded', KILLS_TO_BANNER)

COUNT = int(re.search(r'public static readonly short Count = (\d+);', src).group(1))
rust = rust_header(os.path.basename(__file__))

for name, method, args in re.findall(r'public static \w+\[\] (\w+) = ItemID\.Sets\.Factory\.(CreateBoolSet|CreateIntSet|CreateFloatSet)\((.*?)\);', src):
	args = args.split(', ')
	if args == ['']:
		args = []

	if method == 'CreateBoolSet':
		t = 'bool'
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
		t = 'i32'
		flags = [int('0' if args == [] else args[0])] * COUNT

		for item, value in zip(args[1::2], args[2::2]):
			flags[int(item)] = int(value)
	elif method == 'CreateFloatSet':
		args = [a.replace('f', '') for a in args] # remove f from floats
		t = 'f32'
		flags = [float(args[0])] * COUNT

		for item, value in zip(args[1::2], args[2::2]):
			flags[int(item)] = float(value)

	rust += f'pub const {pascal_to_camel(name)}: &[{t}] = &[{', '.join(str(b).lower() for b in flags)}];\n'

if len(sys.argv) > 2:
	open(sys.argv[2], 'w').write(rust)
else:
	print(rust)
