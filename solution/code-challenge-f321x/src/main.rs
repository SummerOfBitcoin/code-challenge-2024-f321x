pub mod parsing;

fn main() {
    // load transactions,
    // verify transactions, filter out invalid ones
    println!("Hello, world!");
    parsing::parse_transactions_from_dir("/workspaces/code-challenge-2024-f321x/mempool")
}
