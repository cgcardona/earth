use serde::{Deserialize, Serialize};

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
