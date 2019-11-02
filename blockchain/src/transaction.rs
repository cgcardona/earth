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

impl Transaction {
    pub fn new(version: u32, lock_time: u32, inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        Transaction {
            version: version,
            lock_time: lock_time,
            inputs: inputs,
            outputs: outputs,
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
        TxInput::new(1, String::from(s), PrevOut {})
    }
}

impl TxInput {
    pub fn new(sequence: u32, script_sig: String, prev_out: PrevOut) -> Self {
        TxInput {
            sequence: sequence,
            script_sig: script_sig,
            prev_out: prev_out,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrevOut {}

impl From<&'static str> for PrevOut {
    fn from(s: &'static str) -> Self {
        PrevOut::new()
    }
}

impl PrevOut {
    pub fn new() -> Self {
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
        TxOutput::new(1, String::from(s))
    }
}

impl TxOutput {
    pub fn new(value: u64, script_pubkey: String) -> Self {
        TxOutput {
            value: value,
            script_pubkey: script_pubkey,
        }
    }
}
