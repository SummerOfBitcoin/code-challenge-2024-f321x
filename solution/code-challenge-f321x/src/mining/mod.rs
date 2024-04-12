mod assign_parents;
mod packet_weight;

use std::collections::HashMap;
use crate::parsing::transaction_structs::Transaction;
use self::assign_parents::assign_mempool_parents;
use self::packet_weight::calculate_packet_weights;

fn convert_to_hashmap(transactions: &mut Vec<Transaction>) -> HashMap<String, Transaction> {
	let mut txid_tx_map = HashMap::new();

	for transaction in transactions.drain(..) {
		txid_tx_map.insert(transaction.meta.txid_hex.clone(), transaction);
	};
	txid_tx_map
}

pub fn mine_block(valid_transactions: &mut Vec<Transaction>) -> () {
	let mut txid_tx_map = convert_to_hashmap(valid_transactions);

	assign_mempool_parents(&mut txid_tx_map);
	calculate_packet_weights(&mut txid_tx_map);

}
