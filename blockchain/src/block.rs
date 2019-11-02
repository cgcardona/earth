use crate::{BlockHeader, PrevOut, Transaction, TxInput, TxOutput};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub block_header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl From<&'static str> for Block {
    fn from(s: &'static str) -> Self {
        Block {
            block_header: BlockHeader {
                version: 1,
                time: 1,
                bits: 1,
                nonce: 1,
                prev_hash: String::from(s),
                merkle_hash: String::from(s),
            },
            transactions: vec![Transaction {
                version: 1,
                lock_time: 1,
                inputs: vec![TxInput {
                    sequence: 1,
                    script_sig: String::from(s),
                    prev_out: PrevOut {},
                }],
                outputs: vec![TxOutput {
                    value: 1,
                    script_pubkey: String::from(s),
                }],
            }],
        }
    }
}
