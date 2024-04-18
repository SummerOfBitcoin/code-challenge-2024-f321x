use std::collections::{HashMap, HashSet};
use crate::parsing::transaction_structs::Transaction;

pub fn convert_to_hashmap(transactions: Vec<Transaction>) -> HashMap<String, Transaction> {
	let mut txid_tx_map = HashMap::new();

	for transaction in transactions {
		txid_tx_map.insert(transaction.meta.txid_hex.clone(), transaction);
	};
	txid_tx_map
}

pub fn remove_invalid_transactions(transactions: Vec<Transaction>, 
                                    mut invalid_transactions: HashSet<String>) -> HashMap<String, Transaction> {
    let mut transactions = convert_to_hashmap(transactions);
    let mut nothing_removed: bool = false;

    while !nothing_removed {
        nothing_removed = true;

        for (txid, tx) in transactions.iter() {
            for input in &tx.vin {
                if invalid_transactions.contains(&input.txid) {
                    invalid_transactions.insert(txid.clone());  // remove transactions with invalid, unconfirmed parents
                };
            };
        }

        for invalid_txid in &invalid_transactions {
            if transactions.contains_key(invalid_txid) {
                transactions.remove(invalid_txid);
                nothing_removed = false;
            };
        };
    };
    transactions
}