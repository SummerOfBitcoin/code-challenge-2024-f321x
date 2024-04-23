# Summer of Bitcoin 2024 - Felix
In the following document i will explain my solution to the SoB 2024 assignment.

## Design Approach<p style="color:grey;font-size:0.4em;margin-top:0;margin-bottom:0;">-> the story</p>

The program is structured in three main modules and a directory of test scripts:

1. Parsing
2. Validation
3. Block construction
4. Test scripts

I decided to implement the assignment in the Rust programming language because of its known benefits and usage in many bitcoin open source projects, and also because i wanted to learn the language.

### <u>1. Parsing</u>

The parsing module contains the logic to load, parse and deserialize the transaction data from the JSON files contained in the mempool directory into the defined data structures for later use.

The parsing module expects files with **valid JSON format** and will panic if the loaded directory contains invalid files. Parsing the files consists of loading them in a heap allocated *String* variable and deserializing it by using the *Serde JSON* rust crate.

### <u>2. Validation</u>

The validation logic consists of simple **sanity checks** to sort out obviously invalid transactions in a less ressource consuming way and will perform **signature/script** verification of the remaining transactions afterwards.

If a transaction in the mempool is considered invalid is referenced in another transaction (parent transaction), these child transactions will be considered invalid too.

#### *Sanity checks*

The transaction properties are checked on each transaction passed by the parsing module:

* Input and output values
* Input and output count
* Transaction weight
* Validation of txid hash against filename
* Feerate

While calculating the values for verification they will also be stored in the transaction structure for further use (weight, fee, txid).

#### *Script and signature verification*

After a transaction passes the sanity checks the program will call the according signature verification function depending on the transaction type. My solution is able to verify P2PKH and P2WPKH transactions. Other transaction types like P2SH will be considered invalid and could be implemented later.

##### P2PKH
The P2PKH verification function will assemble the validation script from the transaction data and pass it to a script verification submodule able to interpret bitcoin script.


##### P2WPKH
The P2WPKH verification function assembles the transaction commitment accoding to BIP143 and verifies the commitment HASH256 against the witness as well as the ScriptPubKey-pubkey against the HASH160 of the witness pubkey.


### <u>3. Block construction ("mining")</u>

The block construction module expects any amount of valid transactions and will construct a block consisting of header, coinbase transactions and a sorted constellation of transaction ids.

The block construction module will aim to maximise fee revenue respecting the limited block size of 4 000 000 weight units.

Block construction happens in this order:

1. Assigning parents to transactions
2. Calculating packet weights of transactions with their ancestors
3. Sorting transactions aiming at maximum fee revenue
4. Removing transactions with lowest feerate to respect block size limit
5. Assembly of coinbase transaction
	* including construction of wtxid commitment
6. Assembly of block header
	* including hashing to reach target difficulty (the "mining")

After the block data is determined it will be passed to a function storing it in a output.txt file formatted according to the subject requirements.

### <u>4. Test scripts</u>

In the process of writing the program i also used two python scripts to verify some results of the implementation.

#### test_tx_assembly.py
Contains some loose functions to construct a standard p2wpkh transaction commitment.

#### validate_wtxids.py
Script to verify the wtxid construction of my program. Takes a file containing my constructed txids and wtxids and compares them with the correct wtxids pulled from a self hosted mempool.space API. If a wrong wtxid is encountered i can manually debug to find the differences.

## Implementation details<p style="color:grey;font-size:0.4em;margin-top:0;margin-bottom:0;">-> the juice</p>
This section will go trough the program in the same order as the previous one (order of execution) and explain the implementation in more detail assuming understanding of the previous chapter.

### Global
The *Transaction* struct is used to store the transaction data parsed out of the json files by the parsing module.

```
struct Transaction
    meta: 		MetadataStruct,
    version: 	4 byte integer,
    locktime: 	4 byte unsigned integer,
    vins: 		List<TxIn struct>,
    vouts: 		List<TxOut struct>,
```
The *Transaction* struct and the contained sub-structs are defined in **parsing/transaction_structs.rs**.
Most of the runtime the *Transaction* structs are stored and passed between functions in a Vec<_Transaction_>.

Variables contained only in some JSON files are Option<_Some_> variables and some variables are complemented later once available.

The _MetadataStruct_ contained in the Transaction struct contains the following useful transaction metadata:
```
struct MetadataStruct
    json_path: 		Option<absolute path String>,
    txid_hex: 		String,
    wtxid_hex: 		String,
    packet_data: 	Packet,
    weight: 		u64,
    fee: 			u64,
    parents: 		Option<Vec<hex txids>>,
```

### <u>1. Parsing</u>
```
├── parse_transactions_from_dir(directory_path: &str)
│   │
│   └── fs::read_dir(directory_path)
│      └── Iterate over files in the directory
│          │
│          └── parse_file_content(file_to_load: fs::DirEntry)
│              │
│              ├── Check file extension
│              │   └── If not "json", continue to next file
│              │
│              ├── fs::read_to_string(file_path_buf)
│              │   └── Read file content into a String
│              │
│              └── parse_json(&file_content [ref to Sting]) -> using serde json crate
│                  │
│                  ├── from_str::<Transaction>(str_content)
│                  │   └── Deserialize JSON into Transaction struct
│                  │
│                  ├── If deserialization successful
│                  │   ├── Update tx.meta.json_path to absolute json path
│                  │   └── Set input types for each tx.vin
│                  │
│                  └── If deserialization fails
│                      └── Panic with error message -> Invalid JSON file
│
└── Return Vec<Transaction> (parsed transactions)
```
The Vec<_Transaction_> returned by the parsing module is now passed on to the validation module to verify the transactions and sort out invalid ones to be able to construct a valid block.

### <u>2. Validation</u>


## Document your work

- **Implementation Details:** Provide pseudo code of your implementation, including sequence of logic, algorithms and variables used etc.
- **Results and Performance:** Present the results of your solution, and analyze the efficiency of your solution.
- **Conclusion:** Discuss any insights gained from solving the problem, and outline potential areas for future improvement or research. Include a list of references or resources consulted during the problem-solving process.
