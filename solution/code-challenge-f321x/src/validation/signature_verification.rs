use secp256k1::{ecdsa::Signature, Message, PublicKey};
use crate::parsing::transaction_structs::{Transaction, TxIn};
use super::{utils::get_outpoint, utils::double_hash, utils::hash160, ValidationResult};

fn serialize_p2wpkh_scriptcode(txin: &TxIn) -> Vec<u8> {
	let mut scriptcode = Vec::new();
	let mut scriptpubkey_bytes = hex::decode(&txin.prevout.scriptpubkey)
												.expect("Error decoding scriptpubkey hex!");
	scriptcode.extend(hex::decode("1976a914".to_string()).unwrap());
	scriptcode.extend(scriptpubkey_bytes.split_off(2));
	scriptcode.extend(hex::decode("88ac".to_string()).unwrap());
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
	commitment.extend(hex::decode("01000000").unwrap());  // sighash_all <- implement others
	double_hash(&commitment)
}

fn verify_signature_p2wpkh(msg: &[u8], pubkey: &[u8], sig: &[u8]) -> bool {
	let sig = &sig[..sig.len() - 1]; // remove sighash byte
	let sig = Signature::from_der(sig);
	let mut sig = match sig {
		Ok(value) => value,
		Err(err) => {
			println!("Loading DER encoded signature failed: {}", err);
			return false;
		}
	};
	Signature::normalize_s(&mut sig);
	let msg: [u8; 32] = msg.try_into().expect("Commitment hash is not 32 byte!");
	let msg = Message::from_digest(msg);
	let pubkey = PublicKey::from_slice(pubkey).expect("Pubkey invalid!");
	let result = sig.verify(&msg, &pubkey);
	match result {
		Ok(_) => {
			true
		},
		Err(err) => {
			println!("Signature verification failed: {}", err);
			false
		}
	}
}

pub fn verify_p2wpkh(tx: &Transaction, txin: &TxIn) -> ValidationResult {
	let msg = get_segwit_commitment_hash(tx, txin);  // if not ANYONECANPAY (check bip143) sighash types.

	// pubkey verification
	if let Some(witness) = &txin.witness {
		let witness_sig = hex::decode(&witness[0]).expect("Witness sig decoding failed!");
		let witness_pk = hex::decode(&witness[1]).expect("Witness pk hex decoding failed!");
		let witness_pubkey_20bit = hash160(&witness_pk);
		let scriptpubkey_pubkey = hex::decode(txin.prevout.scriptpubkey.clone().split_off(4)).unwrap();
		if  witness_pubkey_20bit ==  scriptpubkey_pubkey {
			verify_signature_p2wpkh(&msg, &witness_pk, &witness_sig);
		} else {
			println!("Pubkeys unequal, witness: {} | scriptpubkey: {}", hex::encode(witness_pubkey_20bit), hex::encode(scriptpubkey_pubkey));
		}
	} else {
		panic!("No witness in p2wpkh!");
	}
	ValidationResult::Valid
}
