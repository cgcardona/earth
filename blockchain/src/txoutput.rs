use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TxOutput {
    pub value: u64,
    pub script_pubkey: String,
}

impl TxOutput {
    pub fn new(value: Option<u64>, script_pubkey: Option<String>) -> Self {
        match (value, script_pubkey) {
            (Some(v), Some(s)) => TxOutput {
                value: v,
                script_pubkey: s,
            },
            (Some(v), None) => TxOutput {
                value: v,
                script_pubkey: Default::default(),
            },
            (None, Some(s)) => TxOutput {
                value: Default::default(),
                script_pubkey: s,
            },
            (None, None) => TxOutput {
                value: Default::default(),
                script_pubkey: Default::default(),
            },
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
