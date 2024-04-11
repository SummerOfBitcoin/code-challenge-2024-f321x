pub mod parsing;
pub mod validation;
pub mod mining;

use parsing::{parse_transactions_from_dir, transaction_structs::{Transaction, TxIn}};
use validation::ValidationResult;
use crate::validation::utils::{get_outpoint, varint, InputType};
// use std::collections::HashSet;

fn main() {
    let mut parsed_transactions = parse_transactions_from_dir("/workspaces/code-challenge-2024-f321x/mempool");

    let mut tx_count = 0;
    // let mut opcodes = Vec::new();
    for tx in &mut parsed_transactions {
      // let set: HashSet<String> = opcodes.into_iter().chain(count_opcodes(tx).into_iter()).collect();
      // opcodes = set.into_iter().collect();
      // println!("EVALUATING: {}", &tx.json_path.as_ref().unwrap());
      // for txin in &tx.vin {
      //   if txin.in_type == InputType::P2SH {
      //     if txin.witness == None {
      //     std::process::exit(0);
      //     }
      //   }
      // }
      match tx.validate() {
        ValidationResult::Valid => {
          tx_count += 1;
          // println!("VALID");
        },
        ValidationResult::Invalid(msg) => {
          // println!("Transaction {:#?} invalid. Reason {}\n", tx.json_path, msg);
          ()
        }
      }
	}
  // println!("{:#?}", opcodes);
  println!("\nDone. Number of parsed transactions: {}\n", tx_count);
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
