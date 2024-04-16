use crate::parsing::transaction_structs::{Transaction, TxIn, TxOut};
use sha2::{Sha256, Digest};
use std::path::Path;
use super::utils::*;
use hex_literal::hex as hexlit;
use super::weight_calculation::is_segwit;

fn get_txid(preimage: &[u8]) -> Vec<u8> {
	let result = double_hash(preimage);
    result.iter().rev().cloned().collect()
}

fn hash_txid(txid: Vec<u8>) -> String {
	let mut hasher = Sha256::new();
	hasher.update(&txid);
    format!("{:x}", hasher.finalize())
}

pub fn serialize_input(input: &TxIn) -> Vec<u8> {
	let mut serialized_input = get_outpoint(input);
	let scriptsig_len = match &input.scriptsig {
		Some(s) => hex::decode(s).expect("Hex decode ss len failed").len(),
		None => 0,
	};
	let scriptsig_len = varint(scriptsig_len as u128);
	let scriptsig_bytes = match &input.scriptsig {
		Some(s) => hex::decode(s).expect("Hex decode ss bytes failed!"),
		None => Vec::new(),
	};
	let sequence_bytes = input.sequence.to_le_bytes();
	serialized_input.extend(scriptsig_len);
	serialized_input.extend(scriptsig_bytes);
	serialized_input.extend_from_slice(&sequence_bytes);
	serialized_input
}

pub fn	serialize_output(output: &TxOut) -> Vec<u8> {
	let mut serialized_output: Vec<u8> = Vec::new();
	let value = output.value.to_le_bytes();
	let pubkey_script_len = match &output.scriptpubkey {
		Some(s) => hex::decode(s).expect("hex decode output s len failed!").len(),
		None => 0,
	};
	let pubkey_script_len = varint(pubkey_script_len as u128);
	let pubkey_script_bytes = match &output.scriptpubkey {
		Some(s) => hex::decode(s).expect("Hex decode output s failed!"),
		None => Vec::new(),
	};
	serialized_output.extend_from_slice(&value);
	serialized_output.extend(pubkey_script_len);
	serialized_output.extend(pubkey_script_bytes);
	serialized_output
}

fn serialize_witnesses_with_amount(tx: &Transaction) -> Vec<u8> {
	let mut witnesses: Vec<u8> = Vec::new();
	let mut witness_amount = 0;
	let mut witness_data: Vec<u8> = Vec::new();

	for input in &tx.vin {
		if let Some(witnesses_hex) = input.witness.as_ref() {
			for witness_element in witnesses_hex {
				witness_amount += 1;

				let witness_element_bytes: Vec<u8> = hex::decode(witness_element).expect("decoding witness hex failed");
				witness_data.extend(varint(witness_element_bytes.len() as u128));
				witness_data.extend(witness_element_bytes);
			};
		};
	};
	witnesses.extend(varint(witness_amount));
	witnesses.extend(witness_data);
	witnesses
}

fn	assemble_txid_preimage(tx: &Transaction, witness: bool) -> Vec<u8> {
	let mut preimage: Vec<u8> = Vec::new();
	let version = tx.version.to_le_bytes();
	let	len_inputs = varint(tx.vin.len() as u128);
	let mut all_input_bytes: Vec<u8> = Vec::new();
	for tx_in in &tx.vin {
		all_input_bytes.append(&mut serialize_input(tx_in));
	}
	let len_outputs = varint(tx.vout.len() as u128);
	let mut all_output_bytes: Vec<u8> = Vec::new();
	for tx_out in &tx.vout {
		all_output_bytes.append(&mut serialize_output(tx_out));
	}
	let locktime = tx.locktime.to_le_bytes();
	preimage.extend_from_slice(&version);
	if witness { preimage.extend(hexlit!("0001")) };
	preimage.extend(len_inputs);
	preimage.extend(all_input_bytes);
	preimage.extend_from_slice(&len_outputs);
	preimage.extend(all_output_bytes);
	if witness {
		preimage.extend(&serialize_witnesses_with_amount(tx));
	}
	preimage.extend_from_slice(&locktime);
	preimage
}

pub fn validate_txid_hash_filename(tx: &mut Transaction) -> bool {
	let tx_preimage = assemble_txid_preimage(tx, false);
	let txid_bytes = get_txid(&tx_preimage);
	let wtxid_bytes: Vec<u8>;

	if is_segwit(tx) {
		let wtx_preimage = assemble_txid_preimage(tx, true);
		wtxid_bytes = get_txid(&wtx_preimage);
	} else {
		wtxid_bytes = txid_bytes.clone();
	}
	tx.meta.txid_hex = hex::encode(&txid_bytes);
	tx.meta.wtxid_hex = hex::encode(&wtxid_bytes);
	let triple_hashed = hash_txid(txid_bytes);
    if let Some(json_path) = tx.meta.json_path.as_ref() {
        let path = Path::new(json_path);
        if let Some(filename) = path.file_stem() {
            if let Some(filename_str) = filename.to_str() {
                return filename_str == triple_hashed;
            }
        }
    }
	false
}
