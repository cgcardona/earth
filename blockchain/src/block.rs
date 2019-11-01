use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub transactions: Vec<String>,
}
