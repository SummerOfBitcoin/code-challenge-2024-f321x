use crate::parsing::transaction_structs::{Transaction, TxIn};
use sha2::{Sha256, Digest};

fn hash(preimage: &[u8]) -> String {
	let mut hasher = Sha256::new();
	hasher.update(preimage);
	let result = hasher.finalize();
	result.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn varint(n: u64) -> Vec<u8> {
    if n <= 252 {
        vec![n as u8]
    } else if n <= 0xffff {
        let mut bytes = vec![0xfd];
        bytes.extend(&(n as u16).to_le_bytes());
        bytes
    } else if n <= 0xffffffff {
        let mut bytes = vec![0xfe];
        bytes.extend(&(n as u32).to_le_bytes());
        bytes
    } else if n <= 0xffffffffffffffff {
        let mut bytes = vec![0xff];
        bytes.extend(&(n as u64).to_le_bytes());
        bytes
    } else {
        panic!("Values larger than 0xffffffffffffffff not supported")
    }
}

fn serialize_input(input: &TxIn) -> Vec<u8> {
	let	mut serialized_input: Vec<u8> = input.txid.as_bytes()
									.iter()
									.rev()
									.cloned()
									.collect();
	let outpoint_index = input.vout.to_le_bytes();
	serialized_input.extend_from_slice(&outpoint_index);
	let scriptsig_len = match &input.scriptsig {
		Some(s) => s.len(),
		None => 0,
	};
	let scriptsig_len = varint(scriptsig_len as u64);
	let scriptsig_bytes = match &input.scriptsig {
		Some(s) => s.as_bytes(),
		None => &[],
	};
	let sequence_bytes = input.sequence.to_le_bytes();
	serialized_input.extend(scriptsig_len);
	serialized_input.extend_from_slice(scriptsig_bytes);
	serialized_input.extend_from_slice(&sequence_bytes);
	serialized_input
}

fn	assemble_txid_preimage(tx: &Transaction) -> Vec<u8> {
	let preimage: Vec<u8> = Vec::new();
	let version = tx.version.to_le_bytes();
	let	len_inputs = varint(tx.vin.len() as u64);

	let mut all_input_bytes: Vec<u8> = Vec::new();
	for tx_in in &tx.vin {
		all_input_bytes.append(&mut serialize_input(tx_in));
	}


	let locktime = tx.locktime.to_le_bytes();
	preimage
}

pub fn validate_txid_hash_filename(tx: &Transaction) -> bool {
	true
}

// # Given arrays of inputs and outputs (no witnesses!) compute the txid.
// # Return the 32 byte txid as a *reversed* hex-encoded string.
// # https://developer.bitcoin.org/reference/transactions.html#raw-transaction-format
// def get_txid(inputs: List[bytes], outputs: List[bytes]) -> str:
//     version = (2).to_bytes(4, "little")
//     locktime = bytes.fromhex("00000000")
//     tx = b""
//     tx += version + len(inputs).to_bytes(1, "little")
//     for input in inputs:
//         tx += input
//     tx += len(outputs).to_bytes(1, "little")
//     for output in outputs:
//         tx += output
//     tx += locktime
//     return hashlib.new("sha256", hashlib.new("sha256", tx).digest()).digest()[::-1].hex()
