pub mod validate_values;
pub mod validate_parsing;
pub mod utils;
mod signature_verification;
mod script;
pub mod weight_calculation;

use crate::parsing::transaction_structs::Transaction;
use self::validate_values::{validate_values_and_set_fee, validate_feerate};
use self::validate_parsing::validate_txid_hash_filename;
use self::utils::InputType;
use self::signature_verification::{verify_p2wpkh, verify_p2pkh}; //, verify_p2sh};
use self::weight_calculation::validate_and_set_weight;

pub enum ValidationResult {
    Valid,
    Invalid(String),
}

fn sanity_checks(tx: &mut Transaction) -> ValidationResult {
	if !validate_values_and_set_fee(tx) {
		return ValidationResult::Invalid("Values don't add up.".to_string());
	}
	if !validate_txid_hash_filename(tx) {
		return ValidationResult::Invalid("Txid does not represent filename!".to_string());
	}
	if !validate_and_set_weight(tx) {
		return ValidationResult::Invalid("Transaction weight too high!".to_string());
	}
	if !validate_feerate(tx) {
		return ValidationResult::Invalid("too low feerate".to_string());
	}
	ValidationResult::Valid
}

fn signature_verification(tx: &Transaction) -> ValidationResult {
	for txin in &tx.vin {
		let tx_type = &txin.in_type;
		let result = match tx_type {
			InputType::P2WPKH => verify_p2wpkh(tx, txin),
			InputType::P2PKH => verify_p2pkh(tx, txin),

			// InputType::P2SH => verify_p2sh(tx, txin),  // fix checkmultisig validation

			// InputType::P2WSH => verify_p2wsh(tx, txin),
			// InputType::P2SH => ValidationResult::Valid, // todo
			_ => {
				// println!("Weird type: {:#?}", tx_type);
				ValidationResult::Invalid("Input type not implemented!".to_string())
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
	pub fn validate(&mut self) -> ValidationResult {
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
