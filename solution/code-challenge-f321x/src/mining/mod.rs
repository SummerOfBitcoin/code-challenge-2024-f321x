mod assign_parents;
mod packet_weight;
mod transaction_sorting;
mod construct_coinbase;
mod header;

use std::collections::HashMap;
use crate::parsing::transaction_structs::Transaction;
use self::{	assign_parents::assign_mempool_parents,
			packet_weight::calculate_packet_weights,
			transaction_sorting::{sort_transactions, cut_size},
			construct_coinbase::{assemble_coinbase_transaction, CoinbaseTxData},
			header::construct_header };

pub struct Block {
	pub header:			String,
	pub coinbase_tx: 	String,
	pub txids: 			Vec<String>,
}

fn return_block(block_header_bytes: &[u8], coinbase_tx: CoinbaseTxData,
				transactions: &Vec<Transaction>) -> Block {
	let header_string = hex::encode(block_header_bytes);
	let coinbase_tx_hex = hex::encode(coinbase_tx.assembled_tx);
	let mut txid_strings: Vec<String> = vec![coinbase_tx.txid_hex];
	for tx in transactions {
		txid_strings.push(tx.meta.txid_hex.clone());
	}
	Block {
		header: header_string,
		coinbase_tx: coinbase_tx_hex,
		txids: txid_strings,
	}
}

pub fn mine_block(txid_tx_map: &mut HashMap<String, Transaction>) -> Block {
	assign_mempool_parents(txid_tx_map);
	calculate_packet_weights(txid_tx_map);

	let block_ordered: Vec<Transaction> = cut_size(sort_transactions(txid_tx_map));

	let coinbase_tx: CoinbaseTxData = assemble_coinbase_transaction(&block_ordered);

	let block_header = construct_header(&block_ordered, &coinbase_tx);

	// for tx in &block_ordered {  // for validation with bitcoin-cli in python script.
	// pipe output in >> wtxids.txt & run python3 validate_wtxids.py
	// 	println!("{},{},{}", tx.meta.txid_hex, tx.meta.wtxid_hex, tx.meta.json_path.as_ref().unwrap());
	// }

	return_block(&block_header, coinbase_tx, &block_ordered)
}
