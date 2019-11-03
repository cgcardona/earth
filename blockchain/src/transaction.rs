use crate::{prevout::PrevOut, txinput::TxInput, txoutput::TxOutput};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub version: u32,
    pub lock_time: u32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

impl Transaction {
    pub fn new(version: u32, lock_time: u32, inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        Default::default()

        // Transaction {
        //     version: version,
        //     lock_time: lock_time,
        //     inputs: inputs,
        //     outputs: outputs,
        // }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            version: Default::default(),
            lock_time: Default::default(),
            inputs: vec![Default::default()],
            outputs: vec![Default::default()],
        }
    }
}

impl From<&'static str> for Transaction {
    fn from(s: &'static str) -> Self {
        let inputs = vec![TxInput {
            sequence: 1,
            script_sig: String::from(s),
            prev_out: PrevOut {},
        }];
        let outputs = vec![TxOutput {
            value: 1,
            script_pubkey: String::from(s),
        }];
        Transaction::new(1, 1, inputs, outputs)
    }
}
