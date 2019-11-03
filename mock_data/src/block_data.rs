use blockchain::{Block, Header, PrevOut, Transaction, TxInput, TxOutput};

pub fn timestamps() -> [u32; 100] {
    [1509343584; 100]
}

pub fn nonces() -> [u64; 3] {
    [3604508752; 3]
}

pub fn bits() -> [&'static str; 3] {
    ["1809b91a"; 3]
}

/// mock block data for tests
// pub fn block_mock_data(iter: u32) -> Block {
pub fn block_mock_data(iter: u32) {
    // let timestamps = timestamps();
    // let b: Block = Default::default();
    // b

    // Block {
    //     header: Header {
    //         version: 536870912,
    //         time: timestamps[0],
    //         bits: iter,
    //         nonce: 3604508752,
    //         prev_hash: String::from(
    //             "0000000000000000043831d6ebb013716f0580287ee5e5687e27d0ed72e6e523",
    //         ),
    //         merkle_hash: String::from(
    //             "4af279645e1b337e655ae3286fc2ca09f58eb01efa6ab27adedd1e9e6ec19091",
    //         ),
    //     },
    //     transactions: vec![Transaction {
    //         version: 1,
    //         lock_time: 1,
    //         inputs: vec![TxInput {
    //             sequence: 1,
    //             script_sig: String::from(""),
    //             prev_out: PrevOut {},
    //         }],
    //         outputs: vec![TxOutput {
    //             value: 1,
    //             script_pubkey: String::from(""),
    //         }],
    //     }],
    // }
}
