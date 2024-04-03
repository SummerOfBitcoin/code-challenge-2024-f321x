// Definition of data structures to hold a bitcoin transaction

use serde::Deserialize;
use serde_with::{serde_as, NoneAsEmptyString};
use crate::validation::utils::get_outpoint;

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct TxOut {
    #[serde_as(as = "NoneAsEmptyString")]
    pub	scriptpubkey:			Option<String>,
	pub	scriptpubkey_asm:		String,
	pub	scriptpubkey_type:		String,
    #[serde_as(as = "NoneAsEmptyString")]
	pub	scriptpubkey_address: 	Option<String>,
	pub	value:					u64,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Script {
	pub	scriptpubkey:			String,
	pub	scriptpubkey_asm:		String,
	pub	scriptpubkey_type:		String,
    #[serde_as(as = "NoneAsEmptyString")]
	pub	scriptpubkey_address: 	Option<String>,
	pub	value:					u64,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct TxIn {
    pub txid:               String,
    pub vout: 	            u32,
    #[serde_as(as = "NoneAsEmptyString")]
    pub scriptsig: 		    Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub scriptsig_asm:      Option<String>,
	pub prevout:			Script,
    pub witness: 			Option<Vec<String>>,
    pub is_coinbase:        bool,
    pub sequence: 			u32,
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub json_path:      Option<String>,
    pub version:        i32,
    pub locktime:       u32,
    pub vin:            Vec<TxIn>,
    pub vout:           Vec<TxOut>,
}

impl Transaction {
    pub fn serialize_all_sequences(&self) -> Vec<u8>{
        let mut all_sequences = Vec::new();
        for input in &self.vin {
            all_sequences.extend(input.sequence.to_le_bytes());
        }
        all_sequences
    }

    pub fn serialize_all_outpoints(&self) -> Vec<u8> {
        let mut all_outpoints = Vec::new();
        for input in &self.vin {
            all_outpoints.extend(get_outpoint(input));
        }
        all_outpoints
    }

    pub fn serialize_all_outputs(&self) -> Vec<u8> {
        let mut all_outputs = Vec::new();
        for output in &self.vout {
            all_outputs.extend(&output.value.to_le_bytes());
            if let Some(scriptpubkey) = &output.scriptpubkey {
                all_outputs.extend(hex::decode(scriptpubkey).unwrap());
            }
        }
        all_outputs
    }
}

// {
//     "version": 1,
//     "locktime": 0,
//     "vin": [
//       {
//         "txid": "9cea74f2834c80e3e9a4cd13cfaed3cf4b9e39fc5e1d222784dda6aeb9fa35e4",
//         "vout": 5,
//         "prevout": {
//           "scriptpubkey": "00148d80fecc4c36bdc5ef58ea5dae8fd7989964ef79",
//           "scriptpubkey_asm": "OP_0 OP_PUSHBYTES_20 8d80fecc4c36bdc5ef58ea5dae8fd7989964ef79",
//           "scriptpubkey_type": "v0_p2wpkh",
//           "scriptpubkey_address": "bc1q3kq0anzvx67utm6cafw6ar7hnzvkfmmeqrnunx",
//           "value": 27393
//         },
//         "scriptsig": "",
//         "scriptsig_asm": "",
//         "witness": [
//           "30440220531de46c024a8e40dbb7d3f8e18cab9091e5e90e2b461e293b9431e5426ac4c202204be3291f88f51b40697be26241860aecbdc8354f01b37acb93422a8eca08aee701",
//           "0390df0d1b67b1dc5613e1e8ebc5cefddd534e8e2f0fc73ed988c1bb6f929a3cd4"
//         ],
//         "is_coinbase": false,
//         "sequence": 4294967295
//       }
//     ],
//     "vout": [
//       {
//         "scriptpubkey": "0014db71541760c7eb2deffd2b438706970eea489387",
//         "scriptpubkey_asm": "OP_0 OP_PUSHBYTES_20 db71541760c7eb2deffd2b438706970eea489387",
//         "scriptpubkey_type": "v0_p2wpkh",
//         "scriptpubkey_address": "bc1qmdc4g9mqcl4jmmla9dpcwp5hpm4y3yu8vmyfvh",
//         "value": 24546
//       }
//     ]
// }
