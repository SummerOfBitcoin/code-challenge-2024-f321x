use std::collections::HashMap;

use crate::parsing::transaction_structs::Transaction;



pub fn sort_transactions_by_value(transactions: &HashMap<String, Transaction>) -> Vec<Transaction> {
    let mut sorted_transactions = Vec::new();

    for (txid, tx) in transactions {

    };
    sorted_transactions
}