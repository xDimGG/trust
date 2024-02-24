# This script takes a C# files that contains the function KillTile_GetItemDrops and converts its large switch statement to rust code
# Example usage from root directory
# python.exe .\scripts\convert_drop_items.py .\TerrariaServer_Decompiled\Terraria\WorldGen.cs .\src\world\transpiled\drop_items.rs

import re
import sys
from tree_sitter import Language, Parser, TreeCursor
import os

base = os.path.dirname(os.path.realpath(__file__))
path = base + '/build/build.so'

Language.build_library(path, [base + '/tree-sitter-c-sharp'])

parser = Parser()
parser.set_language(Language(path, 'c_sharp'))

raw = open(sys.argv[1], encoding='utf-8-sig').read()

block_start = raw.index('switch (tileCache.type)\n      {\n        case 0:')
brace_start = raw.index('{', block_start)

S = 0
for block_len, c in enumerate(raw[brace_start:]):
	if c == '{':
		S += 1
	if c == '}':
		S -= 1
	if S == 0:
		break

switch_block = raw[block_start:brace_start+block_len+1]

# remove fluff
switch_block = switch_block.replace('byte.MaxValue', '255').replace('type', 'id')
src = bytes(re.sub(r'\((u?short|u?int)\) |tileCache\.', '', switch_block), 'utf8')

# get the string value of a node
def s(node):
	return src[node.start_byte:node.end_byte].decode()

tree = parser.parse(src)
cur = tree.walk()

cur.goto_first_child()
cur.goto_first_child()

# cases that can't really be automated or i was too lazy to do
special_cases = set([
	(3, ),
	(5, 596, 616, 634),
	(52, 62, 382),
	(61, 74),
	(71, 72),
	(73, ),
	(83, 84),
	(149, ),
	(171, ),
	(225, ),
	(227, ),
	(314, ),
	(323, ),
	(423, ),
	(428, ),
	(519, ),
	(528, ),
	(571, ),
	(583, ),
	(584, ),
	(585, ),
	(586, ),
	(587, ),
	(588, ),
	(589, ),
	(637, ),
	(650, ),
])

def get_drop_item_eq(cur):
	name = s(cur.node.child(0).child_by_field_name('left'))
	assert name == 'dropItem'
	return s(cur.node.child(0).child_by_field_name('right')).replace('id', 'self.id')

def walk_switch(cur: TreeCursor, match_var, recursive=False, local_name='', arms_only=False, default_value='-1'):
	assert cur.node.type == 'switch_statement'
	cur.goto_last_child()
	cur.goto_first_child()
	if arms_only:
		rust = ''
	else:
		rust = f'match {match_var} {{'
	default_covered = False

	while cur.goto_next_sibling():
		if cur.node.type == '}':
			break

		nums = []

		cur.goto_first_child()
		while cur.node.type == 'case_switch_label':
			nums += [s(cur.node.child(1))]
			cur.goto_next_sibling()

		if cur.node.type == 'default_switch_label':
			nums = ['_']
			cur.goto_next_sibling()
			if cur.node.type != 'return_statement':
				default_covered = True

		match_arm = f'\n\t{' | '.join(map(str, nums))} => '

		if not recursive and tuple(map(int, nums)) in special_cases:
			cur.goto_parent()
			rust += f'{match_arm}{{\n\t\ttodo!()\n\t}}'
			continue

		has_local_var = cur.node.type == 'local_declaration_statement'
		if has_local_var:
			c = cur.node.child(0).child(1).walk()
			assert c.node.type == 'variable_declarator'
			c.goto_first_child()
			local_name = s(c.node)
			c.goto_next_sibling()
			c.goto_last_child()
			c.goto_first_child()

			frame_name = s(c.node)
			assert frame_name in ('frameX', 'frameY')
			c.goto_next_sibling()
			assert c.node.type == '/'
			c.goto_next_sibling()
			divisor = s(c.node)

			name = 'self.frame_x' if frame_name[-1] == 'X' else 'self.frame_y'

			new_default = default_value
			if c.node.type == 'expression_statement':
				new_default = get_drop_item_eq(c)

			cur.goto_next_sibling()

		if cur.node.type == 'expression_statement':
			expr = get_drop_item_eq(cur)
			if has_local_var:
				expr = expr.replace(local_name, f'{name} / {divisor}')
			rust += f'{match_arm}{expr},'
		elif cur.node.type == 'switch_statement':
			rust += match_arm
			if has_local_var:
				c = cur.copy()
				assert c.node.type == 'switch_statement'
				ident = s(c.node.child(2))
				assert ident == local_name # check if we're matching on the thing that just got divided

				rust += f'{{\n\t\tlet {local_name} = {name} / {divisor};\n\t\t'
				rust += walk_switch(cur.copy(), ident, True, local_name, default_value=new_default).replace('\n', '\n\t\t')
				rust += '\n\t}'
			else:
				rust += walk_switch(cur.copy(), f'{name} / {divisor}', True).replace('\n', '\n\t')
		elif cur.node.type == 'if_statement':
			only = None
			scope_default_value = default_value
			while cur.node.type == 'if_statement':
				c = cur.copy()
				c.goto_first_child()
				c.goto_next_sibling()
				c.goto_next_sibling()
				assert c.node.type == 'binary_expression'
				op = s(c.node.child(1))
				l, r = c.node.child(0), c.node.child(2)

				if only is not None:
					assert op in only
					
				if only is None and op in ('!=', '=='):
					only = ('!=', '==')
					if has_local_var:
						rust += f'{match_arm}{{\n\t\tlet {local_name} = {name} / {divisor};'
						assert local_name == s(l)
						rust += f'\n\t\tmatch {local_name} {{'
					else:
						name = 'self.frame_x' if s(l)[-1] == 'X' else 'self.frame_y'
						rust += f'{match_arm}match {name} {{'

				if op == '&&':
					assert s(l.child(0)) == local_name
					assert s(l.child(1)) == '>='
					assert s(r.child(0)) == local_name
					assert s(r.child(1)) == '<='

					c.goto_next_sibling()
					c.goto_next_sibling()
					c.goto_first_child()
					c.goto_next_sibling()

					rust += f'\n\t{s(l.child(2))}..={s(r.child(2))} => {get_drop_item_eq(c)},'
				elif op == '!=':
					c.goto_parent()
					c.goto_next_sibling()
					cur.goto_next_sibling()
					val = get_drop_item_eq(c)
					rust += '\n\t\t'
					if has_local_var:
						rust += '\t'
					rust += f'{s(r)} => {val},'
				elif op == '==':
					c.goto_parent()
					n = c.node.child(4)
					if n.type == 'block':
						true_val = get_drop_item_eq(n.child(1).walk())
					else:
						true_val = get_drop_item_eq(n.walk())
					rust += '\n\t\t'
					if has_local_var:
						rust += '\t'
					rust += f'{s(r)} => {true_val},'
				elif op == '>=':
					c.goto_parent()
					true_val = get_drop_item_eq(c.node.child(4).child(1).walk())
					c.goto_next_sibling()
					false_val = get_drop_item_eq(c)
					rust += f'{match_arm}if self.frame_x >= {s(r)} {{ {true_val} }} else {{ {false_val} }}'
				else:
					assert False
				cur.goto_next_sibling()

			if cur.node.type == 'switch_statement':
				rust += walk_switch(cur.copy(), '', True, local_name, True)
				default_covered = True
			elif cur.node.type == 'expression_statement':
				scope_default_value = get_drop_item_eq(cur)

			if only == ('!=', '=='):
				if has_local_var:
					rust += f'\n\t\t\t_ => {scope_default_value},\n\t\t}}\n\t}}'
				else:
					rust += f'\n\t\t_ => {scope_default_value},\n\t}}'
		elif nums != ['_']:
			print(nums, 'not covered and not a special case')

		cur.goto_parent()

	if not default_covered:
		rust += f'\n\t_ => {default_value},'

	if not arms_only:
		rust += '\n}'

	return rust

out = """use crate::world::tile::Tile;

impl Tile {
	pub fn get_dropped_item(&self) -> i16 {
		%s
	}
}
""" % walk_switch(cur, 'self.id').replace('\n', '\n\t\t')

if len(sys.argv) > 2:
	open(sys.argv[2], 'w').write(out)
else:
	print(out)
