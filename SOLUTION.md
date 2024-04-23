<!-- # Summer of Bitcoin 2024 - Felix -->
In the following document i will explain my solution to the SoB 2024 assignment.

## Design Approach

The program is structured in three main modules:

1. Parsing
2. Validation
3. Block construction

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


### <u>3. Block construction</u>






















## Document your work

Apart from the code, you must also publish a `SOLUTION.md` file explaining your solution in the following format:
- **Design Approach:** Describe the approach you took to design your block construction program, explain all the key concepts of creating a valid block.
- **Implementation Details:** Provide pseudo code of your implementation, including sequence of logic, algorithms and variables used etc.
- **Results and Performance:** Present the results of your solution, and analyze the efficiency of your solution.
- **Conclusion:** Discuss any insights gained from solving the problem, and outline potential areas for future improvement or research. Include a list of references or resources consulted during the problem-solving process.
