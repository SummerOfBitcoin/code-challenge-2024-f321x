pub mod parsing;

fn main() {
    let parsed_transactions = parsing::parse_transactions_from_dir("/workspaces/code-challenge-2024-f321x/mempool");

    let mut amount = 0;
    for tx in &parsed_transactions {
      // for input in &tx.vin{
      //   println!("Is coinbase: {} \n", input.is_coinbase);
      //   if input.is_coinbase == true {
      //     println!("{:?}\n\n", tx);
      //   }
      // }
      println!("{:#?}\n\n", tx);
      amount += 1;
	}
  println!("\nNumber of transactions: {}\n", amount);
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
