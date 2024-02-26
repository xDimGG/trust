# Converts ID/PlayerItemSlotID.cs
import os
import sys
import re
from shared import pascal_to_camel, rust_header

src = open(sys.argv[1], encoding='utf-8-sig').read()

assert 'namespace Terraria.ID' in src
items_file = 'public class ItemID' in src

rust_names = {
	'Inventory0': 'INVENTORY',
	'InventoryMouseItem': 'INVENTORY_MOUSE_ITEM',
	'Armor0': 'ARMOR',
	'Dye0': 'DYES',
	'Misc0': 'MISC',
	'MiscDye0': 'MISC_DYES',
	'Bank4_0': 'BANK',
	'Loadout1_Armor_0': 'ARMOR_LOADOUT_0',
	'Loadout1_Dye_0': 'DYE_LOADOUT_0',
	'Loadout2_Armor_0': 'ARMOR_LOADOUT_1',
	'Loadout2_Dye_0': 'DYE_LOADOUT_1',
	'Loadout3_Armor_0': 'ARMOR_LOADOUT_2',
	'Loadout3_Dye_0': 'DYE_LOADOUT_2',
}

rust = rust_header(os.path.basename(__file__))
offset = 0

for name, size, net_relay in re.findall(r'PlayerItemSlotID\.(\w+) = PlayerItemSlotID\.AllocateSlots\((\d+), (true|false)\);', src):
	# The server doesn't receive this so there's no point in parsing
	if net_relay == 'false':
		offset += int(size)
		continue

	name = rust_names[name]
	name_a = name if size == '1' else f'{name}_START'
	rust += f'pub const {name_a}: usize = {offset};\n'
	offset += int(size)
	if size != '1':
		rust += f'pub const {name}_END: usize = {offset - 1};\n'

rust += '\n'

for name, others in [
	('ARMOR_LOADOUTS', ['ARMOR_LOADOUT_0', 'ARMOR_LOADOUT_1', 'ARMOR_LOADOUT_2']),
	('DYE_LOADOUTS', ['DYE_LOADOUT_0', 'DYE_LOADOUT_1', 'DYE_LOADOUT_2']),
]:
	rust += f'pub const {name}_START: &[usize] = &[{', '.join(f'{o}_START' for o in others)}];\n'
	rust += f'pub const {name}_END: &[usize] = &[{', '.join(f'{o}_END' for o in others)}];\n'

rust += 'pub const LOADOUTS_START: &[usize] = ARMOR_LOADOUTS_START;\n'
rust += 'pub const LOADOUTS_END: &[usize] = DYE_LOADOUTS_END;\n'

if len(sys.argv) > 2:
	open(sys.argv[2], 'w').write(rust)
else:
	print(rust)
