use std::fs;

pub fn read_whole_directory(directory_path: &str) -> Vec<String> {
	let	mut files: Vec<String> = Vec::new();

    for file in fs::read_dir(directory_path).expect("Failed to read directory!") {
		let dir = file.expect("Failed to read entry!");

		let path_str = dir.path().to_str().expect("Conversion to str failed!");
		files.push(path_str.to_string());
    }
	files
}

pub fn parse_transactions_from_dir(directory: &str) -> () {
	let mut transactions: Vec<String> = read_whole_directory(directory);

	for tx in &transactions {
		println!("{}", tx);
	}
}
