mod assign_parents;
mod packet_weight;

use std::collections::HashMap;
use crate::parsing::transaction_structs::Transaction;
use self::assign_parents::assign_mempool_parents;
use self::packet_weight::calculate_packet_weights;




pub fn mine_block(txid_tx_map: &mut HashMap<String, Transaction>) -> () {

	assign_mempool_parents(txid_tx_map);
	calculate_packet_weights(txid_tx_map);

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
