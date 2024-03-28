mod validate_values;
mod validate_parsing;

use crate::parsing::transaction_structs::Transaction;
use self::validate_values::validate_values;
use self::validate_parsing::validate_txid_hash_filename;

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

impl Transaction {
	pub fn validate(&self) -> ValidationResult {
		match sanity_checks(self) {
			ValidationResult::Valid => (),
			ValidationResult::Invalid(msg) => {
				return ValidationResult::Invalid(msg);
			}
		}
		ValidationResult::Valid
	}
}

