use std::time::SystemTime;

#[derive(Debug)]
pub struct Vin {
    coinbase: String,
    sequence: u32,
    n: u32,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Vout {
    value: String,
    n: u32,
    scriptPubKey: ScriptPubKey,
    spentTxId: String,
    spentIndex: u32,
    spentHeight: u32,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct ScriptPubKey {
    hex: String,
    asm: String,
    addresses: Vec<String>,
    // type: String,
    cashAddrs: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Transaction {
    txid: String,
    version: u32,
    locktime: u32,
    vin: Vec<Vin>,
    vout: Vec<Vout>,
    blockhash: String,
    blockheight: u32,
    confirmations: u32,
    time: SystemTime,
    blocktime: SystemTime,
    isCoinBase: bool,
    valueOut: u32,
    size: u32,
}

#[allow(non_snake_case)]
impl Transaction {
    pub fn new(
        txid: String,
        version: u32,
        locktime: u32,
        vin: Vec<Vin>,
        vout: Vec<Vout>,
        blockhash: String,
        blockheight: u32,
        confirmations: u32,
        time: SystemTime,
        blocktime: SystemTime,
        isCoinBase: bool,
        valueOut: u32,
        size: u32,
    ) -> Transaction {
        Transaction {
            txid: txid,
            version: version,
            locktime: locktime,
            vin: vin,
            vout: vout,
            blockhash: blockhash,
            blockheight: blockheight,
            confirmations: confirmations,
            time: time,
            blocktime: blocktime,
            isCoinBase: isCoinBase,
            valueOut: valueOut,
            size: size,
        }
    }
}
