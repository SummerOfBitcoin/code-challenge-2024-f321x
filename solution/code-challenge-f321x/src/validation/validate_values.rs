use crate::parsing::transaction_structs::Transaction;

pub fn validate_values(tx: &Transaction) -> bool {
	let mut input_sum = 0;
	let mut output_sum = 0;

	for txin in &tx.vin {
		input_sum += txin.prevout.value;
	}
	for txout in &tx.vout {
		output_sum += txout.value;
	}
	if input_sum < output_sum {
		return false;
	}
	true
}
