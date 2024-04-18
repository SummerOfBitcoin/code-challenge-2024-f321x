pub mod parsing;
pub mod validation;
pub mod mining;
mod utils_main;

use std::collections::HashSet;
use parsing::{parse_transactions_from_dir, transaction_structs::Transaction};
use validation::ValidationResult;
use utils_main::remove_invalid_transactions;
use mining::{mine_block, Block};
use std::fs::File;
use std::io::prelude::*;

// use std::collections::HashSet;

 // let mut opcodes = Vec::new();
      // let set: HashSet<String> = opcodes.into_iter().chain(count_opcodes(tx).into_iter()).collect();
      // opcodes = set.into_iter().collect();
      // println!("EVALUATING: {}", &tx.meta.json_path.as_ref().unwrap());
      // for txin in &tx.vin {
      //   if txin.in_type == InputType::P2SH {
      //     if txin.witness == None {
      //     std::process::exit(0);
      //     }
      //   }
      // }

      // println!("Transaction {:#?} invalid. Reason {}\n", tx.meta.json_path, msg);
      //()
      // println!("{:#?}", opcodes);

fn output_block(mined_block: Block, output_path: &str) {
  let mut output_file = File::create(output_path).expect("Unable to create output file");

  writeln!(output_file, "{}", mined_block.header).expect("Unable to write to file");
  writeln!(output_file, "{}", mined_block.coinbase_tx).expect("Unable to write to file");

  let len = mined_block.txids.len();
  for (index, tx) in mined_block.txids.iter().enumerate() {
    if index < len - 1 {
        writeln!(output_file, "{}", tx).expect("Unable to write to file");
    } else {
        write!(output_file, "{}", tx).expect("Unable to write to file");
    }
  }
}

fn validate_transactions(parsed_transactions: &mut Vec<Transaction>) -> HashSet<String> {
  let mut invalid_transactions: HashSet<String> = HashSet::new();

  for tx in parsed_transactions {
    match tx.validate() {
      ValidationResult::Valid => { },
      ValidationResult::Invalid(_) => {
        invalid_transactions.insert(tx.meta.txid_hex.clone());
      },
    }
	};
  invalid_transactions
}

fn main() {
  let mut parsed_transactions = parse_transactions_from_dir("../../mempool");
  let invalid_transactions = validate_transactions(&mut parsed_transactions);
  let mut valid_transactions = remove_invalid_transactions(parsed_transactions, invalid_transactions);
  let block = mine_block(&mut valid_transactions);
  output_block(block, "../../output.txt");
  println!("\nDone. Number of mined transactions: {}\n", valid_transactions.len());
}

// Todo:
// Validation: absolute timelocks 1 (unix time), verify sigs
//  Just verify the script and signature.
// Hint: You have to serialize the transaction and then create a commitment hash.

// asm to script pub conversion then checking address == (base58(pubkey) | bech32(pubkey))
// remove dust txs and double spending
// then do script validations like redeem script and their H160 etc same for witnesses
// extract signatures from p2pkh and p2wkh and do ecdsa sig verification
// finally use some ways to fit more txs in a block to maximize fee in total
