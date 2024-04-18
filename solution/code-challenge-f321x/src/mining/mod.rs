mod assign_parents;
mod packet_weight;
mod transaction_sorting;
mod construct_coinbase;
mod header;

use std::collections::HashMap;
use crate::parsing::transaction_structs::Transaction;
use crate::validation::validate_parsing::get_txid;
use self::{	assign_parents::assign_mempool_parents,
			packet_weight::calculate_packet_weights,
			transaction_sorting::{sort_transactions, cut_size},
			construct_coinbase::{assemble_coinbase_transaction, CoinbaseTxData},
			header::construct_header };



pub fn mine_block(txid_tx_map: &mut HashMap<String, Transaction>) -> () {
	assign_mempool_parents(txid_tx_map);
	calculate_packet_weights(txid_tx_map);

	let mut block_ordered: Vec<Transaction> = cut_size(sort_transactions(txid_tx_map));

	let coinbase_tx: CoinbaseTxData = assemble_coinbase_transaction(&block_ordered);

	let block_header = construct_header(&block_ordered, coinbase_tx);
	// for tx in block_order {
	// 	println!("TXID: {} | wTXID: {} \n", &tx.meta.txid_hex, &tx.meta.wtxid_hex);
	// }

	// for tx in &block_order {
	// 	println!{"{} | tx: {}\n weight: {} | packet_weight: {}\nparents:{:?}\ncalc_feerate: {:?}\ntx_json: {}\n", tx.meta.packet_data.packet_feerate_weight,
	// 																tx.meta.txid_hex,
	// 																tx.meta.weight, tx.meta.packet_data.packet_weight,
	// 																tx.meta.parents, validate_feerate(tx),
	// 																tx.meta.json_path.as_ref().unwrap()};
	// }
	// println!("\nlen sorted: {}", block_order.len());

	// for (_, tx) in txid_tx_map.iter() {
	// 	let feerate = tx.meta.fee / tx.meta.weight;
	// 	if feerate == tx.meta.packet_data.packet_feerate_weight {
	// 		println!("Feerate: {} | Packet feerate: {} | same fee/no packet", feerate, tx.meta.packet_data.packet_feerate_weight);
	// 	} else if feerate < tx.meta.packet_data.packet_feerate_weight {
	// 		println!("Feerate: {} | Packet feerate: {} | packet better", feerate, tx.meta.packet_data.packet_feerate_weight);
	// 	} else {
	// 		println!("Feerate: {} | Packet feerate: {} | Packet worse", feerate, tx.meta.packet_data.packet_feerate_weight);
	// 	};
	// }
}
