use blockchain::{Block, BlockHeader, Transaction};

pub fn block_mock_data() -> Block {
    Block {
        block_header: BlockHeader {},
        transactions: vec![Transaction {}],
    }
}
