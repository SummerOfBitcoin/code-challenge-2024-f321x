pub mod parsing;
pub mod validation;

use parsing::{parse_transactions_from_dir, transaction_structs::Transaction};
use validation::ValidationResult;
// use std::collections::HashSet;

// pub fn count_opcodes(tx: &Transaction) -> Vec<String> {
//   let mut opcodes = Vec::new();
//   for txin in &tx.vin {
//     if let Some(inner_witnessscript_asm) = &txin.inner_witnessscript_asm {
//       let v: Vec<String> = inner_witnessscript_asm.split(' ').map(|s| s.to_string()).collect();
//       for opcode in v {
//         if opcode.starts_with("OP_") {
//             opcodes.push(opcode.clone());
//         }
//       }
//     }
//   }
//   opcodes
// }

fn main() {
    let parsed_transactions = parse_transactions_from_dir("/workspaces/code-challenge-2024-f321x/testfiles");

    let mut tx_count = 0;
    // let mut opcodes = Vec::new();
    for tx in &parsed_transactions {
      // let set: HashSet<String> = opcodes.into_iter().chain(count_opcodes(tx).into_iter()).collect();
      // opcodes = set.into_iter().collect();
      match tx.validate() {
        ValidationResult::Valid => (),
        ValidationResult::Invalid(msg) => {
          panic!("Transaction {:#?} invalid. Reason {}\n", tx.json_path, msg);
        }
      }
      tx_count += 1;
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
