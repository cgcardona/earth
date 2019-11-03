use crate::prevout::PrevOut;
use serde::{Deserialize, Serialize};

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
