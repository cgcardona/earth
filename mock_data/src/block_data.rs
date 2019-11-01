use blockchain::{Block, BlockHeader, Transaction, TxInput, TxOutput};

/// mock block data for tests
pub fn block_mock_data() -> Block {
    Block {
        block_header: BlockHeader {
            version: 1,
            time: 1,
            bits: 1,
            nonce: 1,
            prev_hash: String::from(""),
            merkle_hash: String::from(""),
        },
        transactions: vec![Transaction {
            version: 1,
            lock_time: 1,
            inputs: vec![TxInput {}],
            outputs: vec![TxOutput {}],
        }],
    }
}
