use crate::{Header, PrevOut, Transaction, TxInput, TxOutput};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: Option<Header>, transactions: Option<Vec<Transaction>>) -> Self {
        match (header, transactions) {
            (Some(h), Some(t)) => Block {
                header: h,
                transactions: t,
            },
            (Some(h), None) => Block {
                header: h,
                transactions: Default::default(),
            },
            (None, Some(t)) => Block {
                header: Default::default(),
                transactions: t,
            },
            (None, None) => Default::default(),
        }
    }
}

impl Default for Block {
    fn default() -> Self {
        Block {
            header: Default::default(),
            transactions: Default::default(),
        }
    }
}

impl From<&'static str> for Block {
    fn from(s: &'static str) -> Self {
        deserialize(s)
    }
}

fn deserialize(s: &str) -> Block {
    Block {
        header: Header {
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
