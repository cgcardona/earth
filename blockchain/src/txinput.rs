use crate::prevout::PrevOut;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TxInput {
    pub sequence: u32,
    pub script_sig: String,
    pub prev_out: PrevOut,
}

impl TxInput {
    pub fn new(
        sequence: Option<u32>,
        script_sig: Option<String>,
        prev_out: Option<PrevOut>,
    ) -> Self {
        TxInput {
            sequence: sequence.unwrap_or(Default::default()),
            script_sig: script_sig.unwrap_or(Default::default()),
            prev_out: prev_out.unwrap_or(Default::default()),
        }
    }
}

impl Default for TxInput {
    fn default() -> Self {
        TxInput {
            sequence: Default::default(),
            script_sig: Default::default(),
            prev_out: Default::default(),
        }
    }
}

impl From<&'static str> for TxInput {
    fn from(s: &'static str) -> Self {
        TxInput::new(Some(1), Some(String::from(s)), Some(PrevOut {}))
    }
}
