use blockchain::{Block, Header, PrevOut, Transaction, TxInput, TxOutput};

pub fn timestamps() -> [u32; 32] {
    [1509343584; 32]
}

pub fn nonces() -> [u64; 32] {
    [3604508752; 32]
}

pub fn bits() -> [&'static str; 32] {
    ["1809b91a"; 32]
}

/// mock block data for tests
pub fn block_mock_data(iter: u32) -> Block {
    let timestamps = timestamps();

    let version: u32 = 1;
    let time: u32 = timestamps[iter as usize];
    let bits: String = "1809b91a".into();
    let nonce: u64 = 3604508752;
    let prev_hash: String =
        "000000000000000005e14d3f9fdfb70745308706615cfa9edca4f4558332b201".into();
    let merkle_hash: String =
        "4af279645e1b337e655ae3286fc2ca09f58eb01efa6ab27adedd1e9e6ec19091".into();
    // println!("nonces: {:#?}", nonces[iter as usize]);
    // println!("bits: {:#?}", bits[iter as usize]);
    let h: Header = Header::new(version, time, bits, nonce, prev_hash, merkle_hash);
    let p: PrevOut = PrevOut {};

    let tx_input: TxInput = TxInput {
        sequence: 4294967295,
        script_sig: "47304402204420bb9078dedb9b4780f00532a34a5200a2ebfe523646cfad387d12224d0c9302206aa269ed8c300f2114b38ec400baa2568427fc657797c716af157afd84dd7b0e412102960745d3c943d15e7e01a0e84b43591ea0249c2739659d4a2ed09f54ca8e9a3a".into(),
        prev_out: p,
    };

    let tx_output: TxOutput = TxOutput {
        value: 3604508752,
        script_pubkey: "76a91421eac0819a642b1b207fd609afda6505d7abbb4a88ac".into(),
    };

    let tx: Transaction = Transaction {
        version: 2,
        lock_time: 0,
        inputs: vec![tx_input],
        outputs: vec![tx_output],
    };

    let b: Block = Block::new(Some(h), Some(vec![tx]));
    b
}
