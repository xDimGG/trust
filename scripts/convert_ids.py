# So far, this works on ID/TileID.cs and ID/ItemID.cs

import os
import sys
import re
from shared import pascal_to_camel, rust_header

src = open(sys.argv[1], encoding='utf-8-sig').read()

assert 'namespace Terraria.ID' in src

rust = rust_header(os.path.basename(__file__))

for name, id in re.findall(r'public const u?short (\w+) = (-?\d+);', src):
	# Some names already have _ like Fake_BorealWoodChest so just remove them
	name = name.replace('_', '')
	# One-off case
	name = name.replace('newchest', 'NewChest')
	# Convert name to snake case
	name = pascal_to_camel(name, True)

	rust += f'pub const {name}: i16 = {id};\n'

if len(sys.argv) > 2:
	open(sys.argv[2], 'w').write(rust)
else:
	print(rust)
