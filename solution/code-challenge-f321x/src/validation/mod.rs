mod validate_values;
mod validate_parsing;
pub mod utils;
mod signature_verification;
mod script;

use crate::parsing::transaction_structs::Transaction;
use self::validate_values::validate_values;
use self::validate_parsing::validate_txid_hash_filename;
use self::utils::InputType;
use self::signature_verification::{verify_p2wpkh, verify_p2wsh, verify_p2pkh};

pub enum ValidationResult {
    Valid,
    Invalid(String),
}

fn sanity_checks(tx: &Transaction) -> ValidationResult {
	if !validate_values(tx) {
		return ValidationResult::Invalid("Values don't add up.".to_string());
	}
	if !validate_txid_hash_filename(tx) {
		return ValidationResult::Invalid("Txid does not represent filename!".to_string());
	}
	ValidationResult::Valid
}

fn signature_verification(tx: &Transaction) -> ValidationResult {
	for txin in &tx.vin {
		let tx_type = &txin.in_type;
		// let result = match tx_type {
		// 	InputType::P2PKH => {
		// 		println!("p2pkh tx: {}", tx.json_path.as_ref().unwrap());
		// 		std::process::exit(0);
		// 	},
		// 	_ => {
		// 		println!("Weird type: {:#?}", tx_type);
		// 		ValidationResult::Valid
		// 	},
		// };
		let result = match tx_type {
			// InputType::P2WPKH => verify_p2wpkh(tx, txin),
			InputType::P2PKH => verify_p2pkh(tx, txin),

			// InputType::P2WSH => verify_p2wsh(tx, txin),
			// InputType::P2SH => ValidationResult::Valid, // todo
			_ => {
				println!("Weird type: {:#?}", tx_type);
				ValidationResult::Valid
			},
		};
		match result {
			ValidationResult::Valid => (),
			ValidationResult::Invalid(msg) => {
				return ValidationResult::Invalid(msg);
			}
		}
	}
	ValidationResult::Valid
}

impl Transaction {
	pub fn validate(&self) -> ValidationResult {
		match sanity_checks(self) {
			ValidationResult::Valid => (),
			ValidationResult::Invalid(msg) => {
				return ValidationResult::Invalid(msg);
			}
		}
		match signature_verification(self) {
			ValidationResult::Valid => (),
			ValidationResult::Invalid(msg) => {
				return ValidationResult::Invalid(msg);
			}
		}
		ValidationResult::Valid
	}
}
