use secp256k1::{ecdsa::Signature, Message, PublicKey};
use hex_literal::hex as hexlit;
use crate::parsing::transaction_structs::{Transaction, TxIn};
use super::{script::evaluate_script, utils::{double_hash, get_outpoint, hash160, hash_sha256}, ValidationResult};

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

pub fn verify_p2wsh(tx: &Transaction, txin: &TxIn) -> ValidationResult {

	if let Some(witness) = &txin.witness {
		let witness_script_hash = hash_sha256(&hex::decode(&witness.last().unwrap()).expect("Witness sig decoding failed!"));
		let scriptpubkey_hash = hex::decode(txin.prevout.scriptpubkey.clone().split_off(4))
															.expect("Scriptpubkey hash deserialization failed");

		if  witness_script_hash ==  scriptpubkey_hash {
			return ValidationResult::Valid;
			// validate_script()
		} else {
			ValidationResult::Invalid(format!("Witness script last element hash: {} | scriptpubkey_hash_part: {}",
												hex::encode(witness_script_hash), hex::encode(scriptpubkey_hash)))
		}
	} else {
		ValidationResult::Invalid("No witness in transaction!".to_string())
	}
	// ValidationResult::Valid
}

pub fn verify_p2pkh(tx: &Transaction, txin: &TxIn) -> ValidationResult {
	let mut script: Vec<u8> = Vec::new();
	script.extend(hex::decode(txin.scriptsig.as_ref()
														.expect("p2pkh scriptsig empty"))
														.expect("verify p2pkh scriptsig hex decode failed"));
	script.extend(hex::decode(&txin.prevout.scriptpubkey).expect("p2pkh scriptpubkey hec decode failed"));
	let result = evaluate_script(script, txin, tx);
	match result {
		Ok(_) => ValidationResult::Valid,
		Err(err) => ValidationResult::Invalid(err.to_string()),
	}
}

// {
// 	"version": 2,
// 	"locktime": 0,
// 	"vin": [
// 	  {
// 		"txid": "26fecae10ed9f45bc12fb2689d5c09a71c16a72cd35f7c425c1d4208b1f6afe1",
// 		"vout": 1,
// 		"prevout": {
// 		  "scriptpubkey": "76a9141dc07dbc6157fd61c059e714a60a1021dffa49ef88ac",
// 		  "scriptpubkey_asm": "OP_DUP OP_HASH160 OP_PUSHBYTES_20 1dc07dbc6157fd61c059e714a60a1021dffa49ef OP_EQUALVERIFY OP_CHECKSIG",
// 		  "scriptpubkey_type": "p2pkh",
// 		  "scriptpubkey_address": "13iKC5pPN8B7BHikgvkimHojbjUwjg3xs4",
// 		  "value": 123104
// 		},
// 		"scriptsig": "4830450221008ce94ecbd90f24ad4a1c21a78edfb7b328539a21bc820b99bea423bd2626e9c1022023ab569c40b884bc626d1dff17f9098d312831f7e818d8c635e0de38593e0f8f0121035c8fe6ea5a335d8cbdd53dfc14d3f1fccbff0102fbd8efb6f9fd00672c0dc19b",
// 		"scriptsig_asm": "OP_PUSHBYTES_72 30450221008ce94ecbd90f24ad4a1c21a78edfb7b328539a21bc820b99bea423bd2626e9c1022023ab569c40b884bc626d1dff17f9098d312831f7e818d8c635e0de38593e0f8f01 OP_PUSHBYTES_33 035c8fe6ea5a335d8cbdd53dfc14d3f1fccbff0102fbd8efb6f9fd00672c0dc19b",
// 		"is_coinbase": false,
// 		"sequence": 4294967295
// 	  }
// 	],
// 	"vout": [
// 	  {
// 		"scriptpubkey": "001448dfa704897f78fdfbc2b9534055dd9b219ef5a8",
// 		"scriptpubkey_asm": "OP_0 OP_PUSHBYTES_20 48dfa704897f78fdfbc2b9534055dd9b219ef5a8",
// 		"scriptpubkey_type": "v0_p2wpkh",
// 		"scriptpubkey_address": "bc1qfr06wpyf0au0m77zh9f5q4wanvseaadgq9qhf9",
// 		"value": 12465
// 	  },
// 	  {
// 		"scriptpubkey": "76a9141dc07dbc6157fd61c059e714a60a1021dffa49ef88ac",
// 		"scriptpubkey_asm": "OP_DUP OP_HASH160 OP_PUSHBYTES_20 1dc07dbc6157fd61c059e714a60a1021dffa49ef OP_EQUALVERIFY OP_CHECKSIG",
// 		"scriptpubkey_type": "p2pkh",
// 		"scriptpubkey_address": "13iKC5pPN8B7BHikgvkimHojbjUwjg3xs4",
// 		"value": 107963
// 	  }
// 	]
//   }
