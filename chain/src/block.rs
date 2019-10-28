use super::{BlockHeader, Transaction};

#[derive(Debug)]
pub struct Block {
    block_header: BlockHeader,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(block_header: BlockHeader, transactions: Vec<Transaction>) -> Block {
        Block {
            block_header: block_header,
            transactions: transactions,
        }
    }

    pub fn block_header(&self) -> &BlockHeader {
        &self.block_header
    }

    pub fn transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}
