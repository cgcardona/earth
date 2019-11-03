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
        match (sequence, script_sig, prev_out) {
            (Some(s), Some(s_s), Some(p)) => TxInput {
                sequence: s,
                script_sig: s_s,
                prev_out: p,
            },
            (None, Some(s_s), Some(p)) => TxInput {
                sequence: Default::default(),
                script_sig: s_s,
                prev_out: p,
            },
            (Some(s), None, Some(p)) => TxInput {
                sequence: s,
                script_sig: Default::default(),
                prev_out: p,
            },
            (Some(s), Some(s_s), None) => TxInput {
                sequence: s,
                script_sig: s_s,
                prev_out: Default::default(),
            },
            (None, None, Some(p)) => TxInput {
                sequence: Default::default(),
                script_sig: Default::default(),
                prev_out: p,
            },
            (None, Some(s_s), None) => TxInput {
                sequence: Default::default(),
                script_sig: s_s,
                prev_out: Default::default(),
            },
            (Some(s), None, None) => TxInput {
                sequence: s,
                script_sig: Default::default(),
                prev_out: Default::default(),
            },
            (None, None, None) => Default::default(),
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
