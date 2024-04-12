use std::collections::HashMap;
use crate::parsing::transaction_structs::Transaction;

// def add_parent_weight(mempool, child):
//     parent_weight = 0
//     if not mempool[child]["parents"]:
//         return mempool[child]["weight"]
//     for parent in mempool[child]["parents"]:
//         parent_weight += add_parent_weight(mempool, parent)
//     return parent_weight


// def add_parent_fee(mempool, child):
//     parent_fee = 0
//     if not mempool[child]["parents"]:
//         return mempool[child]["fee"]
//     for parent in mempool[child]["parents"]:
//         parent_fee += add_parent_fee(mempool, parent)
//     return parent_fee


// def calculate_packet_values(mempool):
//     for tx in mempool:
//         if mempool[tx]["parents"] is not None:
//            mempool[tx]["packet_weight"] += add_parent_weight(mempool, tx)
//            mempool[tx]["packet_fee"] += add_parent_fee(mempool, tx)
//            mempool[tx]["packet_feerate"] = mempool[tx]["packet_fee"] / mempool[tx]["packet_weight"]


pub fn calculate_packet_weights(transactions: &mut HashMap<String, Transaction>) -> () {

}
