use std::collections::HashMap;

use crate::parsing::transaction_structs::Transaction;


fn get_parent_index(transactions: &Vec<Transaction>, txid: &String) -> usize {
    let mut parent_index: usize = 0;

    for tx in transactions {
        if *tx.meta.txid_hex == *txid {
            break;
        };
        parent_index += 1;
    };
    parent_index
}

fn push_parent_in_front(transactions: &mut Vec<Transaction>, parent_index: usize, child_index: usize) {
    if parent_index < transactions.len() && child_index < transactions.len() {
        let parent = transactions.remove(parent_index);
        transactions.insert(child_index, parent);
    }
}

fn put_parents_in_front(presorted: &mut Vec<Transaction>) {
    let mut nothing_changed: bool = false;

    'outer: while !nothing_changed {
        nothing_changed = true;
        let mut tx_index: usize = 0;

        let transactions_cloned = presorted.clone();
        for tx in transactions_cloned.iter() {
            if let Some(parents) = tx.meta.parents.as_ref() {
                for parent_txid in parents {
                    let parent_index = get_parent_index(presorted, parent_txid);
                    if parent_index > tx_index {
                        push_parent_in_front(presorted, parent_index, tx_index);
                        nothing_changed = false;
                        continue 'outer;
                    };
                };
            };
            tx_index += 1;
        };
    }
}

pub fn sort_transactions(txid_tx_map: &HashMap<String, Transaction>) -> Vec<Transaction> {
    let mut transactions: Vec<&Transaction> = txid_tx_map.values().collect();
    transactions.sort_by(|a, b: &&Transaction|
									b.meta.packet_data.packet_feerate_weight
                                    .cmp(&a.meta.packet_data.packet_feerate_weight));

    let mut sorted_transactions: Vec<Transaction> = transactions.into_iter().cloned().collect();
    put_parents_in_front(&mut sorted_transactions);
    // validate_sorting(&sorted_transactions);
	sorted_transactions
}

fn sigops_amount(_tx: &Transaction) -> i32 {
    // let mut sigops = 0;

    // implement later if sigops cause problems
    // sigops
    0
}

pub fn cut_size(sorted_transactions: Vec<Transaction>) -> Vec<Transaction> {
    let mut block: Vec<Transaction> = Vec::new();
    let mut free_block_space:i64 = 3000000 ; // 4 000 000 - 320 (header) - 1200 (coinbase reserve)
    let mut sigops_left:i32 = 80000;

    for tx in sorted_transactions {
        let sigops = sigops_amount(&tx);

        if free_block_space > tx.meta.weight as i64 && sigops_left > sigops {
            free_block_space -= tx.meta.weight as i64;
            sigops_left -= sigops;
            block.push(tx);
        } else {
            break;
        };
    }
    block
}

// pub fn validate_sorting(sorted_transactions: &Vec<Transaction>) -> () {
//     let mut index = 0;

//     for tx in sorted_transactions {
//         if let Some(parents_txids) = tx.meta.parents.as_ref() {
//             for parent in parents_txids {
//                 let parent_index = get_parent_index(sorted_transactions, parent);
//                 if parent_index >= index {
//                     panic!("Parent after child!");
//                 }
//             }
//         };
//         index += 1;
//     }
// }
