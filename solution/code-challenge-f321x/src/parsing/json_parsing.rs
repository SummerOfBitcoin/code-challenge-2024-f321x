// use json::JsonValue;

// use crate::parsing::transaction_structs::*;

// use serde_json::Value;

// fn parse_script(script_value: &Value) -> Script {
//     Script {
//         scriptpubkey: script_value["scriptpubkey"].as_str().unwrap().to_string(),
//         scriptpubkey_asm: script_value["scriptpubkey_asm"]
//             .as_str()
//             .unwrap()
//             .split(' ')
//             .map(|s| s.to_string())
//             .collect(),
//         scriptpubkey_type: script_value["scriptpubkey_type"].as_str().unwrap().to_string(),
//         scriptpubkey_address: script_value["scriptpubkey_address"].as_str().unwrap().to_string(),
//         value: script_value["value"].as_u64().unwrap(),
//     }
// }

// fn parse_outpoint(outpoint_value: &Value) -> OutPoint {
//     OutPoint {
//         txid: outpoint_value["txid"].as_str().unwrap().to_string(),
//         vout: outpoint_value["vout"].as_u64().unwrap() as u32,
//     }
// }

// fn parse_txin(txin_value: &Value) -> TxIn {
//     TxIn {
//         previous_output: parse_outpoint(&txin_value["prevout"]),
//         script_sig: txin_value["scriptsig"]
//             .as_array()
//             .map(|arr| arr.iter().map(|v| v.as_str().unwrap().to_string()).collect()),
//         prevout: txin_value["prevout"].as_object().map(|v| parse_script(&Value::Object(v.clone()))),
//         sequence: txin_value["sequence"].as_u64().unwrap() as u32,
//         witness: txin_value["witness"]
//             .as_array()
//             .map(|arr| arr.iter().map(|v| v.as_str()    }
// }

// fn parse_txout(txout_value: &Value) -> TxOut {
//     TxOut {
//         value: txout_value["value"].as_u64().unwrap(),
//         script: parse_script(txout_value),
//     }
// }

// fn parse_locktime(locktime_value: &Value) -> LockTime {
//     let locktime = locktime_value.as_u64().unwrap();
//     if locktime < 500000000 {
//         LockTime::Blocks(locktime as u32)
//     } else {
//         LockTime::Seconds(locktime)
//     }
// }

// pub fn json_to_transaction(json_tx: JsonValue) -> Transaction {
// 	Transaction {
//         version: json_tx["version"].as_i32().except("Error in version parsing"),
//         lock_time: parse_locktime(&json_value["locktime"]),
//         input: json_value["vin"]
//             .as_array()
//             .unwrap()
//             .iter()
//             .map(|v| parse_txin(v))
//             .collect(),
//         output: json_value["vout"]
//             .as_array()
//             .unwrap()
//             .iter()
//             .map(|v| parse_txout(v))
//             .collect(),
//     }
// }
