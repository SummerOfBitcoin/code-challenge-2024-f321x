use std::collections::HashMap;
use crate::parsing::transaction_structs::Transaction;
use std::mem;

pub fn assign_mempool_parents(transactions: &mut HashMap<String, Transaction>) -> () {
	let mut parent_transactions: HashMap<String, Vec<String>> = HashMap::new();

	for (txid, tx) in transactions.iter() {
		let mut parents_in_mempool: Vec<String> = Vec::new();

		for input in &tx.vin {
			if transactions.contains_key(&input.txid) {
				parents_in_mempool.push(input.txid.clone());
			}
		};
		if !parents_in_mempool.is_empty() {
            parent_transactions.insert(txid.clone(), parents_in_mempool);
        }
	};

	for (txid, parents) in parent_transactions.iter_mut() {
        if let Some(transaction) = transactions.get_mut(txid) {
            transaction.meta.parents = Some(mem::replace(parents, Vec::new()));
        }
    };
}
