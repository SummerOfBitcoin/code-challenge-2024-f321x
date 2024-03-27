pub mod parsing;

fn main() {
    let parsed_transactions = parsing::parse_transactions_from_dir("/workspaces/code-challenge-2024-f321x/mempool");

    for tx in &parsed_transactions {
		println!("{:?}\n\n", tx);
        // for tx_out in &tx.vout {
        //     if let scriptpubkey_type = &tx_out.scriptpubkey_type {
        //         if scriptpubkey_type == "nulldata" {
        //             std::process::exit(0);
        //         }
        //     }
        // }
	}
}
