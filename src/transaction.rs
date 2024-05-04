use bitcoin::{
    absolute::LockTime, consensus::encode::deserialize_hex, hashes::{serde::Serialize, sha256d::Hash}, transaction::Version, Transaction, TxIn, TxOut, Weight
};

#[derive(Serialize)]
pub struct TransactionResponse {
    pub txid: TxId,
    pub version: Version,
    pub size: Size,
    pub locktime: LockTime,
    pub input: TxInput,
    pub output: TxOutput, 
}

#[derive(Serialize)]
pub struct TxId {
    txid: Hash,
    what_is_txid: String,
    witnesstxid: Hash,
    what_is_witnesstxid: String,
}

#[derive(Serialize)]
pub struct Size {
    base_size: usize,
    what_is_base_size: String,
    size: usize,
    what_is_size: String,
    vsize: usize,
    what_is_vsize: String,
    weight: Weight,
    what_is_weight: String,
    block_weight: String
}

#[derive(Serialize)]
pub struct TxInput {
    explainer: TxInExplainer,
    input: Vec<TxIn>,
}

#[derive(Serialize)]
struct TxInExplainer {
    script_sig: String,
    sequence: String,
    witness: String,
}

#[derive(Serialize)]
pub struct TxOutput {
    explainer: TxOutExplainer,
    output: Vec<TxOut>,
}

#[derive(Serialize)]
struct TxOutExplainer {
    value: String,
    script_pubkey: String,
    // extend script
}

impl From<Transaction> for TransactionResponse {
    fn from(tx: Transaction) -> Self {
        let base_tx_size = tx.base_size();
        let total_tx_size = tx.total_size();
        let tx_weight = tx.weight();

        let u64_tx_weight = u64::from(tx_weight.clone());
        TransactionResponse {
            txid: TxId {
                txid: tx.compute_txid().to_raw_hash(),
                what_is_txid: String::from("Hash of the transaction. Does not include witness data if any"),
                witnesstxid: tx.compute_wtxid().to_raw_hash(),
                what_is_witnesstxid: String::from("Hash of transaction including witness data. Should be equal to txid \
        if legacy transaction. This is the 'hash' field in the 'decoderawtransaction' in bitcoin core")
            },
            version: tx.version,
            size: Size {
                base_size: base_tx_size,
                what_is_base_size: String::from("Transaction size in bytes with witness data stripped"),
                size: total_tx_size,
                what_is_size: String::from("Transaction size in bytes including witness data"),
                vsize: tx.vsize(),
                what_is_vsize: format!("Transaction weight / 4 (rounded up) ---> {} / 4 = {}", tx_weight, u64_tx_weight.div_ceil(4)),
                weight: tx_weight,
                what_is_weight: format!("Base transaction size * 3 + total transaction size ---> {} * 3 + {} = {}", base_tx_size, 
                total_tx_size, base_tx_size * 3 + total_tx_size),
                block_weight: String::from("after segwit upgrade, consensus rule is that block_weight <= 4,000,000")
            },
            locktime: tx.lock_time,
            input: TxInput {
                explainer: TxInExplainer {
                    script_sig: String::from(""),
                    sequence: String::from(""),
                    witness: String::from("")
                },
                input: tx.input
            },
            output: TxOutput {
                explainer: TxOutExplainer {
                    value: String::from(""),
                    script_pubkey: String::from("")
                },
                output: tx.output
            },
        }
    }
}

pub fn get_tx_info(rawtx: String) -> Result<(), String> {
    let tx: Transaction = deserialize_hex(rawtx.as_str()).unwrap();

    let tx = TransactionResponse::from(tx);

    let tx = serde_json::to_string_pretty(&tx).unwrap();
    println!("{}", tx);

    // let tx = serde_json::to_string_pretty(&tx).unwrap();

    Ok(())
}
