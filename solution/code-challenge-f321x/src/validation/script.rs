use std::collections::VecDeque;
use sha2::{Sha256, Digest};
use ripemd::{Ripemd160, Digest as RipemdDigest};

pub fn evaluate_script(script: Vec<u8>) -> bool {
    let mut stack: VecDeque<Vec<u8>> = VecDeque::new();

    // for opcode in script {
    //     match opcode {

    //     }
    // }
    true
}
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
            // 0x52 => { // OP_PUSHBYTES_3
            //     let value = stack.pop_back().unwrap();

// [
//     "OP_CSV",
//     "OP_DUP",
//     "OP_SWAP",
//     "OP_SHA256",
//     "OP_0",
//     "OP_EQUAL",
//     "OP_SIZE",
//     "OP_IFDUP",
//     "OP_CLTV",
//     "OP_ROT",
//     "OP_EQUALVERIFY",
//     "OP_IF",
//     "OP_HASH160",
//     "OP_CHECKSIG",
//     "OP_ELSE",
//     "OP_GREATERTHAN",
//     "OP_OVER",
//     "OP_CHECKMULTISIG",
//     "OP_CHECKSIGVERIFY",
//     "OP_DROP",
//     "OP_ENDIF",
//     "OP_NOTIF",

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

// {
//     "version": 2,
//     "locktime": 0,
//     "vin": [
//       {
//         "txid": "56d47d293e7b4e367125351dfcecd0b6731d5d1a57169a2cf32a17b26685003c",
//         "vout": 1,
//         "prevout": {
//           "scriptpubkey": "a91448420aa8a3a45114def3ca845522116a2fde2ccc87",
//           "scriptpubkey_asm": "OP_HASH160 OP_PUSHBYTES_20 48420aa8a3a45114def3ca845522116a2fde2ccc OP_EQUAL",
//           "scriptpubkey_type": "p2sh",
//           "scriptpubkey_address": "38H5h1TzGM5J5GWLxG1Py8dbDeL7pbwpYa",
//           "value": 142318
//         },
//         "scriptsig": "160014a962b1b69380de037846e608d4628cd8116f8132",
//         "scriptsig_asm": "OP_PUSHBYTES_22 0014a962b1b69380de037846e608d4628cd8116f8132",
//         "witness": [
//           "3044022068db9c3dbfa576a58e108b111075d12f5055038f3a1e71cc27f09a8e9e98f05b022029f5f72796751a99d3e170f3463146635ae56b4113c6f4fbf4885639583fa86c01",
//           "02b38e0a215f1f4a2a32b6326cef6c51cef2101c2dc3f5025ec5600ae9aff68dda"
//         ],
//         "is_coinbase": false,
//         "sequence": 4294967293,
//         "inner_redeemscript_asm": "OP_0 OP_PUSHBYTES_20 a962b1b69380de037846e608d4628cd8116f8132"
//       }
//     ],
//     "vout": [
//       {
//         "scriptpubkey": "5120400784ab7050a09835e3455c855eb9efefed1c29883d3c18fff7e8aa2fc50b29",
//         "scriptpubkey_asm": "OP_PUSHNUM_1 OP_PUSHBYTES_32 400784ab7050a09835e3455c855eb9efefed1c29883d3c18fff7e8aa2fc50b29",
//         "scriptpubkey_type": "v1_p2tr",
//         "scriptpubkey_address": "bc1pgqrcf2ms2zsfsd0rg4wg2h4ealh768pf3q7ncx8l7l525t79pv5sgsu8pu",
//         "value": 27469
//       },
//       {
//         "scriptpubkey": "a91448420aa8a3a45114def3ca845522116a2fde2ccc87",
//         "scriptpubkey_asm": "OP_HASH160 OP_PUSHBYTES_20 48420aa8a3a45114def3ca845522116a2fde2ccc OP_EQUAL",
//         "scriptpubkey_type": "p2sh",
//         "scriptpubkey_address": "38H5h1TzGM5J5GWLxG1Py8dbDeL7pbwpYa",
//         "value": 112902
//       }
//     ]
//   }
