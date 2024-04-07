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

