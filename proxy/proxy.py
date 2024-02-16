import socket
from threading import Thread

# MITM proxy to read data between existing terraria server and client

HOST = '127.0.0.1'  # Standard loopback interface address (localhost)
PORT = 7777  # Port to listen on (non-privileged ports are > 1023)
REAL_PORT = 7778 # Port on which the actual Terraria server is already running

def copy(src, dst, prefix):
	try:
		while True:
			data_size = src.recv(2)
			if not data_size:
				break

			size = int.from_bytes(data_size, 'little') - 2
			data = src.recv(size)
			if not data:
				break

			print(f'{prefix} ({data[0]}) {data[1:].hex()}')
			if data[0] == 32:
				data = bytearray(data)
				data[4:6] = int(999).to_bytes(2, 'little')

			dst.sendall(data_size + data)
	except:
		pass

def start_proxy(client):
	try:
		with client:
			# make real connection
			with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as server:
				server.connect((HOST, REAL_PORT))

				threads = [Thread(target=copy, args=args) for args in ((client, server, '[c->s]:'), (server, client, '[s->c]:'))]
				for t in threads: t.start()
				for t in threads: t.join()

		print(f'Client has disconnected or true server stopped')
	except:
		pass

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
	s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
	s.bind((HOST, PORT))
	s.listen()
	print(f'Proxy started on port {PORT}')

	while True:
		conn, addr = s.accept()
		print(f'Client {addr} has connected')
		t = Thread(target=start_proxy, args=(conn, ))
		t.start()
