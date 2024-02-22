REAL = './dim_real.txt'
TRUST = './dim_fake.txt'
START = '[s->c]: (10) '
HEADER = (4 + 4 + 2 + 2) * 2

sections = {}

for line in open(REAL).readlines():
	line = line.strip().replace('\0', '')
	if line.startswith(START):
		rest = line[len(START):]
		head = rest[:HEADER]
		print(head)
		if head in sections:
			print(head, 'appears more than once')
		sections[head] = rest

def compare(head, real, fake):
	if real == fake:
		print(head, 'is same')
		return
	# print(real)
	# print(fake)
	for i, (a, b) in enumerate(zip(real, fake)):
		if a != b:
			print(head, 'different at byte', i // 2)
			break

for line in open(TRUST).readlines():
	line = line.strip().replace('\0', '')
	if line.startswith(START):
		rest = line[len(START):]
		head = rest[:HEADER]
		if head in sections:
			compare(head, sections[head], rest)
			del sections[head]
		else:
			print(head, 'not in real log')

for k in sections:
	b = bytes.fromhex(k)
	print(k, int.from_bytes(b[:4], 'little'), int.from_bytes(b[4:8], 'little'))
