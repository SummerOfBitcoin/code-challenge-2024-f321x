use secp256k1::{ecdsa::Signature, Message, PublicKey};
use hex_literal::hex as hexlit;
use crate::parsing::transaction_structs::{Transaction, TxIn};
use super::{script::evaluate_script, utils::{double_hash, get_outpoint, hash160}, ValidationResult};

fn serialize_p2wpkh_scriptcode(txin: &TxIn) -> Vec<u8> {
	let mut scriptcode = Vec::new();
	let mut scriptpubkey_bytes = hex::decode(&txin.prevout.scriptpubkey)
												.expect("Error decoding scriptpubkey hex!");
	scriptcode.extend(hexlit!("1976a914"));
	scriptcode.extend(scriptpubkey_bytes.split_off(2));
	scriptcode.extend(hexlit!("88ac"));
	scriptcode
}

fn get_segwit_commitment_hash(tx: &Transaction, txin: &TxIn) -> Vec<u8> {
	let mut commitment = Vec::new();
	commitment.extend(tx.version.to_le_bytes());
	commitment.extend(double_hash(&tx.serialize_all_outpoints()));
	commitment.extend(double_hash(&tx.serialize_all_sequences()));
	commitment.extend(get_outpoint(txin));
	commitment.extend(serialize_p2wpkh_scriptcode(txin));  // prefix len if p2wsh
	commitment.extend(txin.prevout.value.to_le_bytes());
	commitment.extend(txin.sequence.to_le_bytes());
	commitment.extend(double_hash(&tx.serialize_all_outputs()));
	commitment.extend(tx.locktime.to_le_bytes());
	commitment.extend(hexlit!("01000000"));  // sighash_all <- implement others
	double_hash(&commitment)
}

// if len(scriptcode) > 30:
//         result += len(scriptcode).to_bytes(1, "little") + scriptcode

fn verify_signature_p2wpkh(msg: &[u8], pubkey: &[u8], sig: &[u8]) -> ValidationResult {
	let sig = &sig[..sig.len() - 1]; // remove sighash byte
	let sig = Signature::from_der(sig);
	let mut sig = match sig {
		Ok(value) => value,
		Err(err) => {
			return ValidationResult::Invalid(format!("Loading DER encoded signature failed: {}", err));
		}
	};
	Signature::normalize_s(&mut sig);
	let msg: [u8; 32] = msg.try_into().expect("Commitment hash is not 32 byte!");
	let msg = Message::from_digest(msg);
	let pubkey = PublicKey::from_slice(pubkey).expect("Pubkey invalid!");
	let result = sig.verify(&msg, &pubkey);
	match result {
		Ok(_) => ValidationResult::Valid,
		Err(err) => ValidationResult::Invalid(format!("Signature verification failed: {}", err)),
	}
}

pub fn verify_p2wpkh(tx: &Transaction, txin: &TxIn) -> ValidationResult {
	let msg = get_segwit_commitment_hash(tx, txin);  // if not ANYONECANPAY (check bip143) sighash types.
	if let Some(witness) = &txin.witness {
		let witness_sig = hex::decode(&witness[0]).expect("Witness sig decoding failed!");
		let witness_pk = hex::decode(&witness[1]).expect("Witness pk hex decoding failed!");
		let witness_pubkey_20bit = hash160(&witness_pk);
		let scriptpubkey_pubkey = hex::decode(txin.prevout.scriptpubkey.clone().split_off(4))
															.expect("Scriptpubkey pubkey decoding failed");

		if  witness_pubkey_20bit == scriptpubkey_pubkey {
			verify_signature_p2wpkh(&msg, &witness_pk, &witness_sig)
		} else {
			ValidationResult::Invalid(format!("Pubkeys unequal, witness: {} | scriptpubkey: {}",
												hex::encode(witness_pubkey_20bit), hex::encode(scriptpubkey_pubkey)))
		}
	} else {
		ValidationResult::Invalid("No witness in transaction!".to_string())
	}
}

// fn remove_last_data_push_operator(script: &mut Vec<u8>) -> ValidationResult {
// 	let mut last_op_pushdata_index = 0;
// 	let mut index = 0;
// 	let mut pushdata_byte_amount = 0;
// 	let mut last_type_pushdata = false;

// 	while index < script.len() {
// 		if (0x01..=0x4b).contains(&script[index]) {  // OP_PUSHBYTES
// 			last_op_pushdata_index = index;
// 			index += script[index] as usize;
// 			last_type_pushdata = false;
// 		} else if (0x4c..=0x4e).contains(&script[index]) {  // OP_PUSHDATA
// 			last_type_pushdata = true;
// 			last_op_pushdata_index = index;
// 			let byte_amount = match script[index] {
// 				0x4c => {
// 					pushdata_byte_amount = 1;
// 					get_pushdata_amount(&script, pushdata_byte_amount, index)
// 				},
// 				0x4d => {
// 					pushdata_byte_amount = 2;
// 					get_pushdata_amount(&script, 2, index)
// 				},
// 				0x4e =>	{
// 					pushdata_byte_amount = 4;
// 					get_pushdata_amount(&script, 4, index)
// 				},
// 				_ => { return ValidationResult::Invalid("byte amount invalid verify p2wsh".to_string()) }
// 			};
// 			if let Ok(number) = byte_amount {
// 				index += pushdata_byte_amount as usize + number as usize;
// 			} else { return ValidationResult::Invalid("couldn't get byte amount in p2sh 2nd check".to_string())};
// 		}
// 		index += 1;
// 	}
// 	if !last_type_pushdata && last_op_pushdata_index != 0 {
// 		script.remove(last_op_pushdata_index);
// 	} else if last_type_pushdata && last_op_pushdata_index != 0 {
// 		script.remove(last_op_pushdata_index);
// 		for _ in 0..pushdata_byte_amount {
// 			script.remove(last_op_pushdata_index);
// 		}
// 	}
// 	ValidationResult::Valid
// }

// pub fn verify_p2sh(tx: &Transaction, txin: &TxIn) -> ValidationResult {
// 	let mut script: Vec<u8> = Vec::new();
// 	script.extend(hex::decode(txin.scriptsig.as_ref()
// 														.expect("p2psh scriptsig empty"))
// 														.expect("verify p2sh scriptsig hex decode failed"));

// 	script.extend(hex::decode(&txin.prevout.scriptpubkey).expect("p2sh scriptpubkey hex decode failed"));
// 	println!("\nEVAL HASH OF SCRIPT P2SH: {}", hex::encode(&script));
// 	let result = evaluate_script(script, txin, tx);
// 	match result {
// 		Ok(_) => ValidationResult::Valid,
// 		Err(err) => {
// 			println!("INVALID hash verification p2sh");
// 			return ValidationResult::Invalid(err.to_string());
// 		},
// 	};
// 	let mut script: Vec<u8> = Vec::new();
// 	script.extend(hex::decode(txin.scriptsig.as_ref()
// 														.expect("p2psh scriptsig empty"))
// 														.expect("verify p2sh scriptsig hex decode failed"));
// 	match remove_last_data_push_operator(&mut script) {
// 		ValidationResult::Valid => (),
// 		ValidationResult::Invalid(err) => return ValidationResult::Invalid(err),
// 	};
// 	println!("\nEVAL SCRIPT P2SH: {}", hex::encode(&script));
// 	let result = evaluate_script(script, txin, tx);
// 	match result {
// 		Ok(_) => ValidationResult::Valid,
// 		Err(err) => ValidationResult::Invalid(err.to_string()),
// 	}
// }

pub fn verify_p2pkh(tx: &Transaction, txin: &TxIn) -> ValidationResult {
	let mut script: Vec<u8> = Vec::new();
	script.extend(hex::decode(txin.scriptsig.as_ref()
														.expect("p2pkh scriptsig empty"))
														.expect("verify p2pkh scriptsig hex decode failed"));
	script.extend(hex::decode(&txin.prevout.scriptpubkey).expect("p2pkh scriptpubkey hex decode failed"));
	let result = evaluate_script(script, txin, tx);
	match result {
		Ok(_) => ValidationResult::Valid,
		Err(err) => ValidationResult::Invalid(err.to_string()),
	}
}
