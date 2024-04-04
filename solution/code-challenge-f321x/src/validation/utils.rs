use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use crate::parsing::transaction_structs::TxIn;

#[derive(Debug)]
pub enum TransactionType {
	P2TR,
	P2PKH,
	P2SH,
	P2WPKH,
	P2WSH,
	UNKNOWN(String),
}

impl TransactionType {
	pub fn fetch(txin: &TxIn) -> TransactionType {
		let type_string = &txin.prevout.scriptpubkey_type;
		match type_string.as_str() {
			"v1_p2tr" => TransactionType::P2TR,
			"v0_p2wpkh" => TransactionType::P2WPKH,
			"v0_p2wsh" => TransactionType::P2WSH,
			"p2sh" => TransactionType::P2SH,
			"p2pkh" => TransactionType::P2PKH,
			_ => TransactionType::UNKNOWN(type_string.to_string()),
		}
	}
}

pub fn get_outpoint(input: &TxIn) -> Vec<u8> {
	let mut outpoint: Vec<u8> = hex::decode(&input.txid)
										.expect("Failed to decode transaction ID")
										.into_iter()
										.rev()
										.collect();
	let outpoint_index = input.vout.to_le_bytes();
	outpoint.extend_from_slice(&outpoint_index);
	outpoint
}

pub fn hash_sha256(preimage: &[u8]) -> Vec<u8> {
	let mut digest = preimage.to_owned();
	let mut hasher = Sha256::new();

    hasher.update(&digest);
	hasher.finalize_reset().to_vec()
}

pub fn double_hash(preimage: &[u8]) -> Vec<u8> {
	let mut digest = preimage.to_owned();

    for _ in 0..2 {
		digest = hash_sha256(&digest);
    }
	digest
}

pub fn hash160(preimage: &[u8]) -> Vec<u8> {
	let preimage = hash_sha256(preimage);
    let mut hasher = Ripemd160::new();
    hasher.update(&preimage);
    hasher.finalize().to_vec()
}

pub fn varint(n: u128) -> Vec<u8> {
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

// fn validate_ecdsa_signature(msg: String, pubkey: String, sig: String) -> bool {


// 	false
// }

// let message = Message::from_digest_slice(&[0xab; 32]).expect("32 bytes");
// let sig = secp.sign_ecdsa(&message, &secret_key);
// assert_eq!(secp.verify_ecdsa(&message, &sig, &public_key), Ok(()));

// let message = Message::from_digest_slice(&[0xcd; 32]).expect("32 bytes");
// assert_eq!(secp.verify_ecdsa(&message, &sig, &public_key), Err(Error::IncorrectSignature));

// pub fn verify_ecdsa(
//     &self,
//     msg: &Message,
//     sig: &Signature,
//     pk: &PublicKey
// ) -> Result<(), Error>
