import requests
import hashlib
import binascii
import time

def double_sha256(tx_hex):
	tx_bytes = binascii.unhexlify(tx_hex)
	hash1 = hashlib.sha256(tx_bytes).digest()
	hash2 = hashlib.sha256(hash1).digest()
	reversed_hash2 = hash2[::-1]
	return binascii.hexlify(reversed_hash2).decode()

# create tor session of requests to connect to my node at home
def get_tor_session():
    session = requests.session()
    # Tor uses the 9050 port as the default socks port
    session.proxies = {'http':  'socks5h://127.0.0.1:9050',
                       'https': 'socks5h://127.0.0.1:9050'}
    return session

session = get_tor_session()

def parse_text_file():
	data = []
	with open('wtxids.txt', 'r') as file:
		for line in file:
			txid, wtxid, path = line.strip().split(',')
			tx = [txid, wtxid, path]
			data.append(tx)
	return data

def fetch_full_transactions(tx):
	txid = tx[0]
	for attempt in range(10):
		try:
			response = session.get(f"http://my_mempool.space.tor.instance.onion/api/tx/{txid}/hex")
			tx.append(response.text)
			print(response.status_code)
			return response.text
		except Exception as e:
			print(f"Attempt {attempt+1} failed. Retrying in 5 seconds...")
			time.sleep(5)


parsed_file = parse_text_file()

for tx in parsed_file:
	tx_hex = fetch_full_transactions(tx)
	wtxid_rust = tx[1]
	wtxid_fetched = double_sha256(tx_hex)
	if wtxid_fetched != wtxid_rust:
		print("Invalid wtxid:" + tx[2])
		break


