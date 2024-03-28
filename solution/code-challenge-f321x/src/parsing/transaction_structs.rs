// Definition of data structures to hold a bitcoin transaction

use serde::Deserialize;
use serde_with::{serde_as, NoneAsEmptyString};

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct TxOut {
    #[serde_as(as = "NoneAsEmptyString")]
    pub	scriptpubkey:			Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
	pub	scriptpubkey_asm:		Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
	pub	scriptpubkey_type:		Option<String>,
	pub	scriptpubkey_address: 	Option<String>,
	pub	value:					u64,
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Script {
    #[serde_as(as = "NoneAsEmptyString")]
	pub	scriptpubkey:			Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
	pub	scriptpubkey_asm:		Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
	pub	scriptpubkey_type:		Option<String>,
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
	pub prevout:			Option<Script>,
    pub witness: 			Option<Vec<String>>,
    pub is_coinbase:        bool,
    pub sequence: 			u64,
}

#[derive(Deserialize, Debug)]
pub struct Transaction {
    pub version: i32,
    pub locktime: u64,
    pub vin: Vec<TxIn>,
    pub vout: Vec<TxOut>,
}

// {
// 	"version": 2,
// 	"locktime": 0,
// 	"vin": [
// 	  {
// 		"txid": "cda2a6929112c05a5164c6a6029ae71529b022f3c1aee6a76fc00736de570f4a",
// 		"vout": 10,
// 		"prevout": {
// 		  "scriptpubkey": "51201e2d7e631e85167d2f538b89ed2a0e83b2bf209b1a3fe7011c58757a914be6b5",
// 		  "scriptpubkey_asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 1e2d7e631e85167d2f538b89ed2a0e83b2bf209b1a3fe7011c58757a914be6b5",
// 		  "scriptpubkey_type": "v1_p2tr",
// 		  "scriptpubkey_address": "bc1prckhucc7s5t86t6n3wy762swswet7gymrgl7wqgutp6h4y2tu66s4x3q4c",
// 		  "value": 1711
// 		},
// 		"scriptsig": "",
// 		"scriptsig_asm": "",
// 		"witness": [
// 		  "02",
// 		  "750063036f726401010a746578742f706c61696e00367b2270223a226272632d3230222c226f70223a226d696e74222c227469636b223a2261616161222c22616d74223a223130303030227d6851",
// 		  "c1df3132ce091562b5150e601a4643bf625562d04dfdf3e4ede068c09e44ac01ab"
// 		],
// 		"is_coinbase": false,
// 		"sequence": 4294967295
// 	  }
// 	],
// 	"vout": [
// 	  {
// 		"scriptpubkey": "0014a40897ac0756778584e7dbe457cca54abc6daf4c",
// 		"scriptpubkey_asm": "OP_0 OP_PUSHBYTES_20 a40897ac0756778584e7dbe457cca54abc6daf4c",
// 		"scriptpubkey_type": "v0_p2wpkh",
// 		"scriptpubkey_address": "bc1q5syf0tq82emctp88m0j90n99f27xmt6v2f79lx",
// 		"value": 294
// 	  }
// 	]
//   }
