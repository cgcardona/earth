use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TxOutput {
    pub value: u64,
    pub script_pubkey: String,
}

impl TxOutput {
    pub fn new(value: Option<u64>, script_pubkey: Option<String>) -> Self {
        TxOutput {
            value: value.unwrap_or(Default::default()),
            script_pubkey: script_pubkey.unwrap_or(Default::default()),
        }
    }
}

impl Default for TxOutput {
    fn default() -> Self {
        TxOutput {
            value: Default::default(),
            script_pubkey: Default::default(),
        }
    }
}

impl From<&'static str> for TxOutput {
    fn from(s: &'static str) -> Self {
        TxOutput::new(Some(1), Some(String::from(s)))
    }
}
