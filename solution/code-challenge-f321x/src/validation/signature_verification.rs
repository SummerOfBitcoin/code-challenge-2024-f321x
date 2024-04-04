use secp256k1::{ecdsa::Signature, Message, PublicKey};
use hex_literal::hex as hexlit;
use crate::parsing::transaction_structs::{Transaction, TxIn};
use super::{utils::{get_outpoint, double_hash, hash160, hash_sha256}, ValidationResult};

fn serialize_p2wpkh_scriptcode(txin: &TxIn) -> Vec<u8> {
	let mut scriptcode = Vec::new();
	let mut scriptpubkey_bytes = hex::decode(&txin.prevout.scriptpubkey)
												.expect("Error decoding scriptpubkey hex!");
	scriptcode.extend(hexlit!("1976a914"));
	scriptcode.extend(scriptpubkey_bytes.split_off(2));
	scriptcode.extend(hexlit!("88ac"));
	scriptcode
}

// fn serialize_p2wsh_scriptcode(txin: &TxIn) -> Vec<u8> {
// 	let mut p2wsh_scriptcode = Vec::new();


// }

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
			verify_signature_p2wsh(&msg, &witness_pk, &witness_sig)
		} else {
			ValidationResult::Invalid(format!("Witness script last element hash: {} | scriptpubkey_hash_part: {}",
												hex::encode(witness_script_hash), hex::encode(scriptpubkey_hash)))
		}
	} else {
		ValidationResult::Invalid("No witness in transaction!".to_string())
	}
	// ValidationResult::Valid
}

// {
// 	"version": 2,
// 	"locktime": 834636,
// 	"vin": [
// 	  {
// 		"txid": "11bdafffffe2e59d6c901780a20d8a7b660762112b58157f1c6f20e705305be3",
// 		"vout": 0,
// 		"prevout": {
// 		  "scriptpubkey": "00208277c212d2fa741a578d730cd0838cafc62db7558aedef1a24ab960a0a518898",
// 		  "scriptpubkey_asm": "OP_0 OP_PUSHBYTES_32 8277c212d2fa741a578d730cd0838cafc62db7558aedef1a24ab960a0a518898",
// 		  "scriptpubkey_type": "v0_p2wsh",
// 		  "scriptpubkey_address": "bc1qsfmuyykjlf6p54udwvxdpquv4lrzmd643tk77x3y4wtq5zj33zvqpt9a38",
// 		  "value": 72956
// 		},
// 		"scriptsig": "",
// 		"scriptsig_asm": "",
// 		"witness": [
// 		  "3044022032c5730560154cc4a73cde8d0450ffe85a51134723acfa9789aa6b9b062b896a02206783334ed6f1b95ea833361a9a98cf72540d4c3863477bf93759bde492255a8a01",
// 		  "",
// 		  "a9148866a92ac65ad8ef9d3247de2c5e6d4a679e7db1876321038fd7724247548b1d350e721b094389b821dd07f6cea1bc6aee2298ab3708b2f267022001b2752102e1ed24f0f0ef10fa7986932dd7d139525698a64783a9c55f32257d94898934ba68ac"
// 		],
// 		"is_coinbase": false,
// 		"sequence": 288,
// 		"inner_witnessscript_asm": "OP_HASH160 OP_PUSHBYTES_20 8866a92ac65ad8ef9d3247de2c5e6d4a679e7db1 OP_EQUAL OP_IF OP_PUSHBYTES_33 038fd7724247548b1d350e721b094389b821dd07f6cea1bc6aee2298ab3708b2f2 OP_ELSE OP_PUSHBYTES_2 2001 OP_CSV OP_DROP OP_PUSHBYTES_33 02e1ed24f0f0ef10fa7986932dd7d139525698a64783a9c55f32257d94898934ba OP_ENDIF OP_CHECKSIG"
// 	  }
// 	],
// 	"vout": [
// 	  {
// 		"scriptpubkey": "001436dd72acc2b6165e6edd00716c37622d23c87bf7",
// 		"scriptpubkey_asm": "OP_0 OP_PUSHBYTES_20 36dd72acc2b6165e6edd00716c37622d23c87bf7",
// 		"scriptpubkey_type": "v0_p2wpkh",
// 		"scriptpubkey_address": "bc1qxmwh9txzkct9umkaqpckcdmz953us7lhgfjccx",
// 		"value": 70684
// 	  }
// 	]
//   }
