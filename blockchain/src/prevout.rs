use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PrevOut {}

impl PrevOut {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for PrevOut {
    fn default() -> Self {
        PrevOut {}
    }
}

impl From<&'static str> for PrevOut {
    fn from(s: &'static str) -> Self {
        PrevOut::new()
    }
}
