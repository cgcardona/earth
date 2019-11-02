use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub version: u32,
    pub lock_time: u32,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxInput {
    pub sequence: u32,
    pub script_sig: String,
    pub prev_out: PrevOut,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrevOut {}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxOutput {
    pub value: u64,
    pub script_pubkey: String,
}
