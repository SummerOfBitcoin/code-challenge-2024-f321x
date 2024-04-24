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

### <u>2.1 Transaction validation - Sanity checks</u>

#### ***Input and output values and count***

```fn validate_values_and_set_fee(tx: &mut Transaction) -> bool```

Validates that the transaction has higher sum of input values than output values (no "inflation"), also checks if the transaction even has inputs and outputs and that the values are possible (below 21m bitcoin).

If the all checks pass the fee will be stored in the passed mutable _Transaction_ reference.

#### ***Parsing validation***

```fn validate_txid_hash_filename(tx: &mut Transaction) -> bool ```

Compares the SHA256 hash of the TXID against the filename of the JSON file to verify correct parsing of the data.

To get the TXID the transaction has to be byte serialized without witness parts (no marker, flag and witnesses) in the specified structure below:

1. Version [4 bytes LE]
2. (WTXID: 1 byte marker & 1 byte flag)
3. Input count [varint LE]
4. All serialized inputs each consisting of:
    1. Outpoint txid in natural bytes (txid referenced in input)
    2. Outpoint index [4 bytes LE]
    3. Scriptsig length [varint LE bytes]
    4. Scriptsig bytes
    5. Sequence bytes [4 bytes LE]
5. Output count [varint LE]
6. All serialized outputs each consisting of:
    1. Value in satoshi [8 bytes LE]
    2. Length of output scriptpubkey [varint LE bytes]
    3. Output scriptpubkey bytes
7. (WTXID: serialized witnesses)
8. Locktime [4 bytes LE]

Afterwards this bytes stored in a Vec<_u8_> can be double SHA256 hashed and compared with the filenames, if they are unequal there would be some problem in the transaction parsing or serialization.

The functions in validate_parsing.rs will also calculate the WTXID due to the similar logic and store it alongside the TXID and store it in the mutable _Transaction_ reference for later use.

#### ***Transaction weight***

``` fn validate_and_set_weight(tx: &mut Transaction) -> bool ```

Weight units define the "size" of the transaction used later for calculation of the feerate and priorization of the transaction by the miner. Weight units discount some parts of the transaction so it can't be compared to bytes.

This are the multipliers used when calculating the transaction weight from its byte size:

| Field   | Multiplier |
|---------|------------|
| version | x4         |
| marker  | x1         |
| flag    | x1         |
| input   | x4         |
| output  | x4         |
| witness | x1         |
| locktime| x4         |

Calculating the weight is done by decoding all parts stored in the _Transaction_ into bytes and calculating the sum of all parts each multiplied by its weight multiplier. If it is a segwit transaction marker, flag and witness are included in the calculation too as they are stored on the blockchain as well.

As part of the sanity check the function *validate_and_set_weight(tx: &mut Transaction)* will check if the weight of the transaction is above 4 000 000 WU (- 320 WU for the block header & - 400 WU reserve for the coinbase transaction) which would be too large to be included in any block.

Afterwards the transaction weight will be stored in the mutable _Transaction_ reference for later use.

#### ***Transaction feerate***
``` fn validate_feerate(tx: &Transaction) -> bool ```

The last simple sanity check is now able to calculate the feerate from the previously calculated weight and transaction fee. The **Bitcoin Core** implementation of bitcoin will only relay transactions with a feerate above 1 satoshi per vbyte. A virtual byte is another unit of size and can be calculated by dividing the weight by 4.

```
vbyte_size = tx.meta.weight / 4
feerate = tx.meta.fee / vbyte_size
    if feerate < 1
        return false
return true
```

Although transactions with a feerate below 1 sat/vbyte are not strictly invalid they would have to be mined out of band by a miner as they won't be stored in the mempool so i will consider them invalid in the program.

### <u>2.2 Transaction validation - Signature and Script verification</u>
Out of the available transaction types in the given mempool i decided to implement verification for P2PKH and P2WPKH and consider other transaction types invalid.

To verify contained scripts and to learn the function of *Bitcoin Script*, the "language" used to specify and satisfy the spending conditions of transaction outputs i implemented a *Script* verification "engine" located in validation/script.rs.

The function *evaluate_script()* goes trough the script byte by byte and calls the according function if an opcode is encountered. The stack is implemented as a VecDeque<_Vec<*u8*>_> data structure.
```
fn evaluate_script(
    script: Vec<u8>,
    txin: &TxIn,
    tx: &Transaction, ) -> Result<(), Box<dyn Error>>
```
This are the opcodes supported by the function:

| Hex | OP_NAME | Function Call |
|-----|---------|---------------|
| 0xa8 | OP_SHA256 | `stack.push_back(hash_sha256(&last))` |
| 0xa9 | OP_HASH160 | `stack.push_back(hash160(&last))` |
| 0x75 | OP_DROP | `stack.pop_back()` |
| 0x7c | OP_SWAP | `op_swap(&mut stack)?` |
| 0x00 | OP_0 | `stack.push_back(Vec::new())` |
| 0x76 | OP_DUP | `stack.push_back(last.clone())` |
| 0x87 | OP_EQUAL | `op_equal(&mut stack)?` |
| 0x7b | OP_ROT | `op_rot(&mut stack)?` |
| 0x82 | OP_SIZE | `op_size(&mut stack)?` |
| 0x78 | OP_OVER | `op_over(&mut stack)?` |
| 0xa0 | OP_GREATERTHAN | `op_greaterthan(&mut stack)?` |
| 0x88 | OP_EQUALVERIFY | `op_equalverify(&mut stack)?` |
| 0x73 | OP_IFDUP | `op_ifdup(&mut stack)?` |
| 0xb2 | OP_CHECKSEQUENCEVERIFY | `op_checksequenceverify(&mut stack, txin, tx)?` |
| 0xb1 | OP_CHECKLOCKTIMEVERIFY | `op_checklocktimeverify(&mut stack, tx, txin)?` |
| 0xac | OP_CHECKSIG | `op_checksig(&mut stack, tx, txin)?` |
| 0x74 | OP_DEPTH | `op_depth(&mut stack)?` |
| 0xad | OP_CHECKSIGVERIFY | `op_checksig(&mut stack, tx, txin)?; op_verify(&mut stack)?` |
| 0x51..=0x60 | OP_PUSHNUM (1-16) | `op_pushnum(&mut stack, opcode)?` |
| 0x4f | OP_1NEGATE | `stack.push_back(vec![255])` |
| 0x01..=0x4b | OP_PUSHBYTES | `op_pushbytes(&mut stack, &mut index, &script)?` |
| 0x4c | OP_PUSHDATA1 | `op_pushdata(&mut stack, 1, &mut index, &script)?` |
| 0x4d | OP_PUSHDATA2 | `op_pushdata(&mut stack, 2, &mut index, &script)?` |
| 0x4e | OP_PUSHDATA4 | `op_pushdata(&mut stack, 4, &mut index, &script)?` |
| 0xae | OP_CHECKMULTISIG | `op_checkmultisig(&mut stack, tx, txin)?` |


#### P2PKH
```
fn verify_p2pkh(tx: &Transaction, txin: &TxIn) -> ValidationResult
```
When the main verifying loop detects the transaction input type as P2PKH the function above will assemble a script from the transaction input data with a structure similar to this (but in bytes):
```
scriptSig part
------------
OP_PUSHBYTES
SIGNATURE
OP_PUSHBYTES
PUBLIC_KEY
------------
+
ScriptPubKey part
-------------------------
OP_DUP
OP_HASH160
OP_PUSHBYTES_20
PUBLIC KEY HASH (HASH160)
OP_EQUALVERIFY
OP_CHECKSIG
------------------------
```

The script will then be passed to the script verification function which will return the result.

If any transaction input is invalid the transaction will be considered invalid.

#### P2WPKH
My P2WPKH verification is more hardcoded as i implemented the Script engine afterwards and could be refactored to use the script engine as further improvement.

I first assemble the commitment to generate HASH256(commitment) message for signatue verification according to the BIP143 serialization specification:

1. Version [4-byte little endian]
2. hashPrevouts [*HASH256(tx.serialize_all_outpoints())*]
3. hashSequence [*HASH256(tx.serialize_all_sequences())*]
4. outpoint [32-byte outpoint txid natural byte order + 4-byte little endian index]
5. scriptCode of the input (byte serialized scriptcode)
6. value of the output spent by this input (8-byte little endian)
7. Sequence of the input (4-byte little endian)
8. hashOutputs [*HASH256(tx.serialize_all_outputs())*]
9. Locktime of the transaction (4-byte little endian)
10. sighash type of the signature [4-byte little endian, *SIGHASH_ALL hardcoded*]

Then the program compares if HASH160(witness public key) is equal to the public key encoded in the ScriptPubKey. If so the commitment hash is verified against the signature and public key using ecdsa on secp256k1 (imported as rust crate).


#### All transaction considered invalid according to the previous tests will be stored be stored in a HashSet in form of their hex txid. Afterwards all Transactions contained in the HashSet will be removed from the Vec<*Transaction*> of parsed transactions returning a Vec of valid transactions only.

### <u>3. Block construction</u>


## Document your work

- **Implementation Details:** Provide pseudo code of your implementation, including sequence of logic, algorithms and variables used etc.
- **Results and Performance:** Present the results of your solution, and analyze the efficiency of your solution.
- **Conclusion:** Discuss any insights gained from solving the problem, and outline potential areas for future improvement or research. Include a list of references or resources consulted during the problem-solving process.
