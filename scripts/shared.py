import re
from tree_sitter import Language, Parser
import os

base = os.path.dirname(os.path.realpath(__file__))

# Path always relative to this py file
so_file = base + '/build/build.so'
gh_repo = base + '/tree-sitter-c-sharp'

# Build the library if it is not already built
Language.build_library(so_file, [gh_repo])

# Create a parser using the library
parser = Parser()
parser.set_language(Language(so_file, 'c_sharp'))

def pascal_to_camel(name, upper=False):
	name = '_'.join(re.findall(r'[A-Z0-9]+[a-z]*?(?=[A-Z0-9]|$)', name))
	return name.upper() if upper else name.lower()

def find_block(src, search):
	block_start = src.index(search)
	brace_start = src.index('{', block_start)

	S = 0
	for block_len, c in enumerate(src[brace_start:]):
		if c == '{':
			S += 1
		if c == '}':
			S -= 1
		if S == 0:
			break

	return src[block_start:brace_start+block_len+1]

def rust_header(file):
	return "// This file was auto-generated by scripts/%s\n#![allow(dead_code)]\n\n" % file
