import hashlib
from typing import List

# Given an output script as a byte array, compute the p2wsh witness program
# This is a segwit version 0 pay-to-script-hash witness program.
# https://github.com/bitcoin/bips/blob/master/bip-0141.mediawiki#p2wsh
def get_p2wsh_program(script: bytes, version: int=0) -> bytes:
    version_byte = version.to_bytes(1, "little")
    # Compute the SHA256 hash of the script
    hash256  = hashlib.new("sha256", script).digest()
    # Prepend the version byte and return
    return version_byte + hash256

# Given an outpoint, return a serialized transaction input spending it
# Use hard-coded defaults for sequence and scriptSig
def input_from_utxo(txid: bytes, index: int, scriptsig: bytes, sequence: int) -> bytes:
    # Reverse the txid hash so it's little-endian
    reversed_txid = txid[::-1]
    # Index of the output being spent (zero-indexed)
    index = index.to_bytes(4, "little")
    outpoint = reversed_txid + index
    print("\nInput outpoint hex: " + outpoint.hex() + "\n")
    # ScriptSig (empty)
    scriptsig_len = len(scriptsig).to_bytes(1, "little")
    print("\nScriptsig length: " + scriptsig_len.hex() + "\n")
    # Sequence (default)
    sequence = sequence.to_bytes(4, "little")
    # Return the full input
    return outpoint + scriptsig_len + scriptsig + sequence

# Given an output script and value (in satoshis), return a serialized transaction output
def output_from_options(script: bytes, value: int) -> bytes:
    value = value.to_bytes(8, "little")
    script_length = len(script).to_bytes(1, "little")
    return value + script_length + script

# Given arrays of inputs and outputs (no witnesses!) compute the txid.
# Return the 32 byte txid as a *reversed* hex-encoded string.
# https://developer.bitcoin.org/reference/transactions.html#raw-transaction-format
def get_txid(inputs: List[bytes], outputs: List[bytes]) -> str:
    version = (1).to_bytes(4, "little")
    print("version: " + version.hex())
    tx = b""
    tx += version + len(inputs).to_bytes(1, "little")
    print("Amount TxIN: " + len(inputs).to_bytes(1, "little").hex() + "\n")
    for input in inputs:
        tx += input
        print("appended input bytes: " + input.hex() + "\n")
    tx += len(outputs).to_bytes(1, "little")
    print("Amount TxOUT: " + len(outputs).to_bytes(1, "little").hex() + "\n")
    for output in outputs:
        tx += output
        print("appended output bytes: " + output.hex() + "\n")
    locktime = bytes.fromhex("00000000")
    tx += locktime
    print("Locktime: " + locktime.hex() + "\n")
    print(tx.hex())
    return hashlib.new("sha256", hashlib.new("sha256", tx).digest()).digest()[::-1].hex()


def spend_p2wpkh():

    serialized_input = input_from_utxo(bytes.fromhex("d1283ec7f6a2bcb65a5905033168258ca282e806c9dc7164415519a5ef041b14"),
                                          0,
                                          bytes.fromhex("4730440220200b9a61529151f9f264a04e9aa17bb6e1d53fb345747c44885b1e185a82c17502200e41059f8ab4d3b3709dcb91b050c344b06c5086f05598d62bc06a8b746db4290121025f0ba0cdc8aa97ec1fffd01fac34d3a7f700baf07658048263a2c925825e8d33"),
                                          4294967295)



    output = output_from_options(bytes.fromhex("76a914e5977cf916acdba010b9d847b9682135aa3ea81a88ac"), 1100665)

    # Reserialize without witness data and double-SHA256 to get the txid
    txid = get_txid([serialized_input], [output])
    print(txid)
    return hashlib.new("sha256", bytes.fromhex(txid)).digest().hex()

print(spend_p2wpkh())


# {
#   "version": 1,
#   "locktime": 0,
#   "vin": [
#     {
#       "txid": "d1283ec7f6a2bcb65a5905033168258ca282e806c9dc7164415519a5ef041b14",
#       "vout": 0,
#       "prevout": {
#         "scriptpubkey": "76a91496bc8310635539000a65a7cc95cb773c0cc7009788ac",
#         "scriptpubkey_asm": "OP_DUP OP_HASH160 OP_PUSHBYTES_20 96bc8310635539000a65a7cc95cb773c0cc70097 OP_EQUALVERIFY OP_CHECKSIG",
#         "scriptpubkey_type": "p2pkh",
#         "scriptpubkey_address": "1Ek2BpKHUbr6SrrWq4P3Tf2jB6UCST2bwx",
#         "value": 1103367
#       },
#       "scriptsig": "4730440220200b9a61529151f9f264a04e9aa17bb6e1d53fb345747c44885b1e185a82c17502200e41059f8ab4d3b3709dcb91b050c344b06c5086f05598d62bc06a8b746db4290121025f0ba0cdc8aa97ec1fffd01fac34d3a7f700baf07658048263a2c925825e8d33",
#       "scriptsig_asm": "OP_PUSHBYTES_71 30440220200b9a61529151f9f264a04e9aa17bb6e1d53fb345747c44885b1e185a82c17502200e41059f8ab4d3b3709dcb91b050c344b06c5086f05598d62bc06a8b746db42901 OP_PUSHBYTES_33 025f0ba0cdc8aa97ec1fffd01fac34d3a7f700baf07658048263a2c925825e8d33",
#       "is_coinbase": false,
#       "sequence": 4294967295
#     }
#   ],
#   "vout": [
#     {
#       "scriptpubkey": "76a914e5977cf916acdba010b9d847b9682135aa3ea81a88ac",
#       "scriptpubkey_asm": "OP_DUP OP_HASH160 OP_PUSHBYTES_20 e5977cf916acdba010b9d847b9682135aa3ea81a OP_EQUALVERIFY OP_CHECKSIG",
#       "scriptpubkey_type": "p2pkh",
#       "scriptpubkey_address": "1MvyDWhroVV7BAL1twmwvY88DdvBEmPbG7",
#       "value": 1100665
#     }
#   ]
# }

# Rust:
# 01000000013431623134306665356139313535313434363137636439633630386532383261633835323836313333303530393561353662636232613666376365333832316400000000d43437333034343032323032303062396136313532393135316639663236346130346539616131376262366531643533666233343537343763343438383562316531383561383263313735303232303065343130353966386162346433623337303964636239316230353063333434623036633530383666303535393864363262633036613862373436646234323930313231303235663062613063646338616139376563316666666430316661633334643361376637303062616630373635383034383236336132633932353832356538643333ffffffff0179cb10000000000032373661393134653539373763663931366163646261303130623964383437623936383231333561613365613831613838616300000000
