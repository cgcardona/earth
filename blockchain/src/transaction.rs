use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub version: u32,
    pub lock_time: u32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

impl From<&'static str> for Transaction {
    fn from(s: &'static str) -> Self {
        Transaction {
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
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxInput {
    pub sequence: u32,
    pub script_sig: String,
    pub prev_out: PrevOut,
}

impl From<&'static str> for TxInput {
    fn from(s: &'static str) -> Self {
        TxInput {
            sequence: 1,
            script_sig: String::from(s),
            prev_out: PrevOut {},
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrevOut {}

impl From<&'static str> for PrevOut {
    fn from(s: &'static str) -> Self {
        PrevOut {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxOutput {
    pub value: u64,
    pub script_pubkey: String,
}

impl From<&'static str> for TxOutput {
    fn from(s: &'static str) -> Self {
        TxOutput {
            value: 1,
            script_pubkey: String::from(s),
        }
    }
}
