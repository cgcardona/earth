use serde::{Deserialize, Serialize};

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
