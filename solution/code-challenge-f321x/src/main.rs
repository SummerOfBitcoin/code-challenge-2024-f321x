pub mod parsing;
pub mod validation;

use parsing::parse_transactions_from_dir;
use validation::ValidationResult;

fn main() {
    let parsed_transactions = parse_transactions_from_dir("/workspaces/code-challenge-2024-f321x/mempool");

    let mut tx_count = 0;
    for tx in &parsed_transactions {
      match tx.validate() {
        ValidationResult::Valid => (),
        ValidationResult::Invalid(msg) => {
          println!("Transaction {:#?} invalid. Reason {}\n", tx.json_path, msg);
          std::process::exit(1);
        }
      }
      tx_count += 1;
	}
  println!("\nDone. Number of parsed transactions: {}\n", tx_count);
}

// Todo:
// Validation: absolute timelocks 1 (unix time), verify sigs
//  Just verify the script and signature.
// Hint: You have to serialise the transaction and then create a commitment hash.

// asm to script pub conversion then checking address == (base58(pubkey) | bech32(pubkey))
// remove dust txs and double spending
// then do script validations like redeem script and their H160 etc same for witnesses
// extract signatures from p2pkh and p2wkh and do ecdsa sig verification
// finally use some ways to fit more txs in a block to maximize fee in total
