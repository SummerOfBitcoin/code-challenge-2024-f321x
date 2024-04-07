use std::collections::VecDeque;
use std::error::Error;

use crate::parsing::transaction_structs::{Transaction, TxIn};

use super::utils::{hash160, hash_sha256, decode_num};

fn op_swap(stack: &mut VecDeque<Vec<u8>>) -> Result<(), &'static str> {
    if stack.len() >= 2 {
        let len = stack.len();
        let last = stack.get_mut(len - 1).expect("OP_SWAP last!");
        let second_last = stack.get_mut(len - 2).expect("OP_SWAP second last!");
        std::mem::swap(last, second_last);
        return Ok(());
    }
    Err("OP_SWAP stack < 2")
}

fn op_equal(stack: &mut VecDeque<Vec<u8>>) -> Result<(), &'static str> {
    if stack.len() >= 2 {
        let last = decode_num(&stack.pop_back().expect("Unwrap op_equal"));
        let second_last = decode_num(&stack.pop_back().expect("OP_Equal"));
        if last == second_last {
            stack.push_back(vec![1u8]);
            return Ok(());
        }
    } else {
        return Err("OP_EQUAL stack len < 2");
    }
    stack.push_back(Vec::new());
    Ok(())
}

fn op_rot(stack: &mut VecDeque<Vec<u8>>) -> Result<(), &'static str> {
    if stack.len() >= 3 {
        let third_item = stack.pop_back().expect("OP_ROT");
        let second_item = stack.pop_back().expect("OP_ROT");
        let first_item = stack.pop_back().expect("OP_ROT");
        stack.push_back(second_item);
        stack.push_back(first_item);
        stack.push_back(third_item);
        return Ok(());
    }
    Err("OP_ROT stack len < 3")
}

fn op_size(stack: &mut VecDeque<Vec<u8>>) -> Result<(), &'static str> {
    if stack.len() > 0 {
        if let Some(last) = stack.back() {
            let length = last.len();
            let length_bytes = length.to_le_bytes().to_vec();
            stack.push_back(length_bytes);
            return Ok(());
        } else {
            return Err("OP_SIZE getting last element failed");
        }
    }
    Err("OP_SIZE stack empty")
}

fn op_over(stack: &mut VecDeque<Vec<u8>>) -> Result<(), &'static str> {
    let stack_len = stack.len();
    if stack_len >= 2 {
        if let Some(second_element) = stack.get(stack_len - 2) {
            stack.push_back(second_element.clone());
            return Ok(());
        } else {
            return Err("OP_OVER getting second element failed");
        }
    }
    Err("OP_OVER stack < 2")
}

fn op_greaterthan(stack: &mut VecDeque<Vec<u8>>) -> Result<(), &'static str> {
    let stack_size = stack.len();
    if stack_size >= 2 {
        if let Some(b) = stack.pop_back() {
            if let Some(a) = stack.pop_back() {
                let a = decode_num(&a);
                let b = decode_num(&b);
                if a > b {
                    stack.push_back(vec![1u8]);
                } else {
                    stack.push_back(Vec::new());
                }
                return Ok(());
            } else { return Err("OP_GREATERTHAN second element pop failed"); }
        } else { return Err("OP_GREATERTHAN first element pop failed"); }
    }
    Err("OP_GREATERTHAN stack < 2")
}

fn op_equalverify(stack: &mut VecDeque<Vec<u8>>) -> Result<(), &'static str> {
    op_equal(stack)?;
    if let Some(bool) = stack.pop_back() {
        if bool.is_empty() { 
            return Err("Equalverify false"); 
        } else { return Ok(()) };
    } else { return Err("OP_EQUALVERIFY stack pop failed") };
}

fn op_ifdup(stack: &mut VecDeque<Vec<u8>>) -> Result<(), &'static str> {
    let length = stack.len();
    if length < 1 { return Err("OP_IFDUP length < 1") };
    if let Some(last_item) = stack.get(length - 1) {
        if last_item.is_empty() {
            return Ok(());
        } else {
            stack.push_back(last_item.clone());
        }
        return Ok(());
    } else { return Err("OP_IFDUP getting last element failed") };
}

// Marks transaction as invalid if the relative lock time of the input (enforced by BIP 0068 with nSequence) 
// is not equal to or longer than the value of the top stack item. The precise semantics are described in BIP 0112. 
// Assume relative locktimes are valid
fn op_checksequenceverify(stack: &mut VecDeque<Vec<u8>>, txin: &TxIn, tx: &Transaction) -> Result<(), &'static str> {
// check tx version >= 2
    let sequence = txin.sequence;
    let disable_flag = 1 << 31;
    let locktime_mask = 0x0000ffff;
    let time_flag = 1 << 22;
    if stack.is_empty() { return Err("OP_CSV stack empty") };

    if let Some(locktime_element) = stack.pop_back() {
        let number = decode_num(&locktime_element) as u32;
        if number < 0 || locktime_element.is_empty() { return Err("OP_CSV number < 0 or empty") };

        if (number & disable_flag) == 0 {
            if tx.version < 2 { return Err("OP_CSV Transaction version is less than 2.") };
            if (sequence & disable_flag) != 0 { return Err("OP_CSV Transaction input sequence number disable flag is set.") };
            if (number & time_flag) != (sequence & time_flag) { return Err("OP_CSV Relative lock-time types are not the same.") };

            let locktime_sequence = sequence & locktime_mask;
            let locktime_stack = number & locktime_mask;
            if locktime_stack > locktime_sequence { return Err("OP_CSV Stack > Sequence LT")};
        }
    } else { return Err("OP_CSV time pop from stack failed.")}
    Ok(())
}

fn op_checklocktimeverify(stack: &mut VecDeque<Vec<u8>>) -> Result<(), &'static str> {
    
    Ok(())
}

pub fn evaluate_script(data: Vec<Vec<u8>>, script: Vec<u8>, txin: &TxIn, tx: &Transaction) -> Result<(), Box<dyn Error>> {
    let mut stack: VecDeque<Vec<u8>> = VecDeque::new();
    // let mut flow_stack: Vec<Flow> = Vec::new();

    for element in data {
        stack.push_back(element);
    }
    for opcode in script {
        match opcode {
            0xa8 => {   // OP_SHA256
                if let Some(last) = stack.pop_back() {
                    stack.push_back(hash_sha256(&last));
                } else {
                    return Err("OP_SHA256 stack empty".into());
                }
            },
            0xa9 => {  // OP_HASH160
                if let Some(last) = stack.pop_back() {
                    stack.push_back(hash160(&last));
                } else {
                    return Err("OP_HASH160 stack empty".into());
                }
            } 
            0x76 => if stack.pop_back().is_none() { return Err("OP_DROP stack empty".into()) }, // OP_DROP 
            0x7c => op_swap(&mut stack)?,
            0x00 => stack.push_back(Vec::new()),    // OP_0
            0x76 => {                       // OP_DUP
                if let Some(last) = stack.back() {
                    stack.push_back(last.clone());
                } else {
                    return Err("OP_DUP stack empty.".into())
                }                                  
            }
            0x87 => op_equal(&mut stack)?, // OP_EQUAL
            0x7b => op_rot(&mut stack)?,    // OP_ROT
            0x82 => op_size(&mut stack)?, // OP_SIZE
            0x78 => op_over(&mut stack)?, // OP_OVER
            0xa0 => op_greaterthan(&mut stack)?, // OP_GREATERTHAN
            0x88 => op_equalverify(&mut stack)?, // OP_EQUALVERIFY
            0x73 => op_ifdup(&mut stack)?, // OP_IFDUP
            0xb2 => op_checksequenceverify(&mut stack, txin, tx)?, // OP_CSV
            0xb1 => op_checklocktimeverify(&mut stack)?, // OP_CLTV
            // 0x63 => if !op_if(&mut stack) { return false },  // OP_IF
            // 0x68 => // OP_ENDIF
        };
    }
    Ok(())
}


//     "OP_CLTV",

//     "OP_IF",
//     "OP_ELSE",
//     "OP_ENDIF",
//     "OP_NOTIF",

//     "OP_CHECKSIG",
//     "OP_CHECKMULTISIG",
//     "OP_CHECKSIGVERIFY",

//     "OP_PUSHBYTES_2",
//     "OP_PUSHBYTES_1",
//     "OP_PUSHNUM_2",
//     "OP_PUSHBYTES_33",
//     "OP_PUSHBYTES_32",
//     "OP_PUSHBYTES_3",
//     "OP_PUSHNUM_1",
//     "OP_PUSHNUM_5",
//     "OP_PUSHNUM_6",
//     "OP_PUSHBYTES_20",
//     "OP_PUSHBYTES_4",
//     "OP_PUSHNUM_4",
//     "OP_PUSHNUM_10",
//     "OP_PUSHNUM_3",
//     "OP_PUSHNUM_16",
// ]

            // 0x63 => { // OP_IF
            //     let condition = stack.pop_back().unwrap();
            //     if condition.is_empty() {
            //         // Skip to the corresponding OP_ELSE or OP_ENDIF
            //         let mut depth = 1;
            //         while depth > 0 {
            //             let op = stack.pop_back().unwrap();
            //             if op == vec![0x67] { // OP_ELSE
            //                 if depth == 1 {
            //                     break;
            //                 }
            //                 depth -= 1;
            //             } else if op == vec![0x68] { // OP_ENDIF
            //                 depth -= 1;
            //             } else if op == vec![0x63] { // OP_IF
            //                 depth += 1;
            //             }
            //         }
            //     }
            // }

// enum Flow {
//     IF,
//     ELSE,
//     END
// }

// fn op_if(stack: &mut VecDeque<Vec<u8>>) -> bool {
//     if let Some(condition) = stack.pop_back() {
//         if condition.is_empty() {
//             // go to else or endif
//             while !stack.is_empty() {
//                 if let Some(instruction) = stack.pop_back() {
//                     if instruction == vec![0x67]  {  // OP_ENDIF
//                         break;
//                     } else if instruction == vec![0x68] { //OP_ELSE

//                     }
//                 } else { return false };
//             }

//         } else { return true };
//     } else { return false };
// }

// {
// 	"version": 2,
// 	"locktime": 834636,
// 	"vin": [
// 	  {
// 		"txid": "11bdafffffe2e59d6c901780a20d8a7b660762112b58157f1c6f20e705305be3",
// 		"vout": 0,
// 		"prevout": {
// 		  "scriptpubkey": "00208277c212d2fa741a578d730cd0838cafc62db7558aedef1a24ab960a0a518898",
// 		  "scriptpubkey_asm": "OP_0 OP_PUSHBYTES_32 8277c212d2fa741a578d730cd0838cafc62db7558aedef1a24ab960a0a518898",
// 		  "scriptpubkey_type": "v0_p2wsh",
// 		  "scriptpubkey_address": "bc1qsfmuyykjlf6p54udwvxdpquv4lrzmd643tk77x3y4wtq5zj33zvqpt9a38",
// 		  "value": 72956
// 		},
// 		"scriptsig": "",
// 		"scriptsig_asm": "",
// 		"witness": [
// 		  "3044022032c5730560154cc4a73cde8d0450ffe85a51134723acfa9789aa6b9b062b896a02206783334ed6f1b95ea833361a9a98cf72540d4c3863477bf93759bde492255a8a01",
// 		  "",
// 		  "a9148866a92ac65ad8ef9d3247de2c5e6d4a679e7db1876321038fd7724247548b1d350e721b094389b821dd07f6cea1bc6aee2298ab3708b2f267022001b2752102e1ed24f0f0ef10fa7986932dd7d139525698a64783a9c55f32257d94898934ba68ac"
// 		],
// 		"is_coinbase": false,
// 		"sequence": 288,
// 		"inner_witnessscript_asm": "OP_HASH160 OP_PUSHBYTES_20 8866a92ac65ad8ef9d3247de2c5e6d4a679e7db1 OP_EQUAL OP_IF OP_PUSHBYTES_33 038fd7724247548b1d350e721b094389b821dd07f6cea1bc6aee2298ab3708b2f2 OP_ELSE OP_PUSHBYTES_2 2001 OP_CSV OP_DROP OP_PUSHBYTES_33 02e1ed24f0f0ef10fa7986932dd7d139525698a64783a9c55f32257d94898934ba OP_ENDIF OP_CHECKSIG"
// 	  }
// 	],
// 	"vout": [
// 	  {
// 		"scriptpubkey": "001436dd72acc2b6165e6edd00716c37622d23c87bf7",
// 		"scriptpubkey_asm": "OP_0 OP_PUSHBYTES_20 36dd72acc2b6165e6edd00716c37622d23c87bf7",
// 		"scriptpubkey_type": "v0_p2wpkh",
// 		"scriptpubkey_address": "bc1qxmwh9txzkct9umkaqpckcdmz953us7lhgfjccx",
// 		"value": 70684
// 	  }
// 	]
//   }


            // 0x63 => { // OP_IF
            //     let condition = stack.pop_back().unwrap();
            //     if condition.is_empty() {
            //         // Skip to the corresponding OP_ELSE or OP_ENDIF
            //         let mut depth = 1;
            //         while depth > 0 {
            //             let op = stack.pop_back().unwrap();
            //             if op == vec![0x67] { // OP_ELSE
            //                 if depth == 1 {
            //                     break;
            //                 }
            //                 depth -= 1;
            //             } else if op == vec![0x68] { // OP_ENDIF
            //                 depth -= 1;
            //             } else if op == vec![0x63] { // OP_IF
            //                 depth += 1;
            //             }
            //         }
            //     }
            // }
            // 0x64 => { // OP_NOTIF
            //     let condition = stack.pop_back().unwrap();
            //     if !condition.is_empty() {
            //         // Skip to the corresponding OP_ELSE or OP_ENDIF
            //         let mut depth = 1;
            //         while depth > 0 {
            //             let op = stack.pop_back().unwrap();
            //             if op == vec![0x67] { // OP_ELSE
            //                 if depth == 1 {
            //                     break;
            //                 }
            //                 depth -= 1;
            //             } else if op == vec![0x68] { // OP_ENDIF
            //                 depth -= 1;
            //             } else if op == vec![0x63] { // OP_IF
            //                 depth += 1;
            //             }
            //         }
            //     }
            // }
            // 0x67 => { // OP_ELSE
            //     // Skip to the corresponding OP_ENDIF
            //     let mut depth = 1;
            //     while depth > 0 {
            //         let op = stack.pop_back().unwrap();
            //         if op == vec![0x68] { // OP_ENDIF
            //             depth -= 1;
            //         } else if op == vec![0x63] { // OP_IF
            //             depth += 1;
            //         }
            //     }
            // }
            // 0x68 => { // OP_ENDIF
            //     // Do nothing
            // }
            // 0x76 => { // OP_DUP
            //     let value = stack.back().unwrap().clone();
            //     stack.push_back(value);
            // }
            // 0x7c => { // OP_SWAP
            //     let value1 = stack.pop_back().unwrap();
            //     let value2 = stack.pop_back().unwrap();
            //     stack.push_back(value1);
            //     stack.push_back(value2);
            // }
            // 0xa8 => { // OP_SHA256
            //     let mut hasher = Sha256::new();
            //     let value = stack.pop_back().unwrap();
            //     hasher.update(value);
            //     let hash = hasher.finalize();
            //     stack.push_back(hash.to_vec());
            // }
            // 0xa9 => { // OP_HASH160
            //     let mut hasher = Ripemd160::new();
            //     let value = stack.pop_back().unwrap();
            //     hasher.update(value);
            //     let hash = hasher.finalize();
            //     stack.push_back(hash.to_vec());
            // }
            // 0xac => { // OP_CHECKSIG
            //     let pubkey = stack.pop_back().unwrap();
            //     let signature = stack.pop_back().unwrap();
            //     // Perform signature verification here
            //     // For simplicity, always return true
            //     stack.push_back(vec![1]);
            // }
            // 0xad => { // OP_CHECKSIGVERIFY
            //     let pubkey = stack.pop_back().unwrap();
            //     let signature = stack.pop_back().unwrap();
            //     // Perform signature verification here
            //     // For simplicity, always return true
            // }
            // 0xae => { // OP_CHECKMULTISIG
            //     let num_pubkeys = stack.pop_back().unwrap()[0] as usize;
            //     let mut pubkeys = Vec::new();
            //     for _ in 0..num_pubkeys {
            //         pubkeys.push(stack.pop_back().unwrap());
            //     }
            //     let num_signatures = stack.pop_back().unwrap()[0] as usize;
            //     let mut signatures = Vec::new();
            //     for _ in 0..num_signatures {
            //         signatures.push(stack.pop_back().unwrap());
            //     }
            //     // Perform multi-signature verification here
            //     // For simplicity, always return true
            //     stack.push_back(vec![1]);
            // }
            // 0x50 => { // OP_PUSHBYTES_1
            //     let value = stack.pop_back().unwrap();
            //     stack.push_back(value);
            // }
            // 0x51 => { // OP_PUSHBYTES_2
            //     let value = stack.pop_back().unwrap();
            //     stack.push_back(value);
            // }
            //     let value = stack.pop_back().unwrap();
            // 0x52 => { // OP_PUSHBYTES_3