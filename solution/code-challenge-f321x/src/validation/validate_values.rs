use crate::parsing::transaction_structs::Transaction;

pub fn validate_values_and_set_fee(tx: &mut Transaction) -> bool {
	let mut input_sum = 0;
	let mut output_sum = 0;

	for txin in &tx.vin {
		input_sum += txin.prevout.value;
	}
	for txout in &tx.vout {
		output_sum += txout.value;
	}
	if input_sum < output_sum {  // no inflation!
		return false;
	}
	if input_sum > (20999999 * 100000000) || output_sum > (20999999 * 100000000) {  // this is unrealistic
		return false;
	};
	tx.fee = input_sum - output_sum;
	true
}

pub fn validate_and_set_feerate(tx: &Transaction) -> bool {
	let vbyte_size = tx.weight / 4;
	let feerate = tx.fee as u64 / vbyte_size as u64;
	if feerate < 1 {
		return false;
	}
	return true;
}
