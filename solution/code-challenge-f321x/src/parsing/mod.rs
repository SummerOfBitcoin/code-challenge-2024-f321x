// functions to load the Json files from the specified directory in a Vec of Json objects (json crate)

pub mod transaction_structs;

use std::fs;
use self::transaction_structs::Transaction;
use serde_json::from_str;
use crate::validation::utils::InputType;

fn parse_json(str_content: &str) -> Option<Transaction> {
	let tx = from_str::<Transaction>(str_content);
	if let Ok(tx) = tx {
		return Some(tx);
	}
	println!("{:#?}", tx.err());
	None
}

fn parse_file_content(file_to_load: fs::DirEntry) -> Option<Transaction> {
	let file_path_buf = file_to_load.path();

	if file_path_buf.extension().expect("Invalid file extension") != "json" {
		println!("Invalid file extension: {}, continuing...", file_path_buf.as_path().display());
		return None;
	}
	let file_content = fs::read_to_string(file_path_buf.as_path()).expect("Reading file content failed");

	match parse_json(&file_content) {
		Some(mut tx) => {
			tx.json_path = Some(file_path_buf.as_path()
										.to_str()
										.expect("Path to string conversion failed!")
										.to_string());
			for txin in &mut tx.vin {
				InputType::fetch_type(txin);
			}
			Some(tx)
		},
		None => {
			panic!("Invalid Json content in file: {:?}, Delete or correct this file!\n", file_path_buf);
		}
	}
}

pub fn parse_transactions_from_dir(directory_path: &str) -> Vec<Transaction> {
	let	mut transactions: Vec<Transaction> = Vec::new();

    for file in fs::read_dir(directory_path).expect("Failed to read directory!") {
		let dir_entry = file.expect("Failed to read file entry!");
		if let Some(transaction) = parse_file_content(dir_entry) {
			transactions.push(transaction);
		}
    }
	transactions
}
