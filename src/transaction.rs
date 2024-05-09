use bitcoin::{
    absolute::LockTime,
    consensus::encode::deserialize_hex,
    hashes::{serde::Serialize, sha256d::Hash},
    transaction::Version,
    Amount, ScriptBuf, Sequence, Transaction, TxIn, TxOut, Txid, Weight, Witness,
};

#[derive(Serialize)]
pub struct TransactionResponse {
    pub txid: TxId,
    pub version: Version,
    pub size: Size,
    pub locktime: LockTime,
    pub what_is_locktime: String,
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
    block_weight: String,
}

#[derive(Serialize)]
pub struct TxInput {
    explainer: TxInExplainer,
    inputs: Vec<TxInResponse>,
}

#[derive(Serialize)]
struct TxInExplainer {
    txid: String,
    vout: String,
    script_sig: String,
    sequence: String,
    witness: String,
}

#[derive(Serialize)]
struct TxInResponse {
    txid: Txid,
    vout: u32,
    script_sig: ScriptBuf,
    sequence: Sequence,
    witness: Witness,
}

impl From<TxIn> for TxInResponse {
    fn from(tx_in: TxIn) -> Self {
        TxInResponse {
            txid: tx_in.previous_output.txid,
            vout: tx_in.previous_output.vout,
            script_sig: tx_in.script_sig,
            sequence: tx_in.sequence,
            witness: tx_in.witness,
        }
    }
}

#[derive(Serialize)]
pub struct TxOutput {
    explainer: TxOutExplainer,
    output: Vec<TxOutResponse>,
}

#[derive(Serialize)]
struct TxOutExplainer {
    value: String,
    script_pubkey: String,
    script: String,
}

#[derive(Serialize)]
struct TxOutResponse {
    sats_value: Amount,
    btc_value: String,
    script_pubkey: ScriptBuf,
    script: String,
    script_type: String,
}

impl From<TxOut> for TxOutResponse {
    fn from(tx_out: TxOut) -> Self {
        let output_script = tx_out.script_pubkey.clone();
        let script_type = {
            if output_script.is_p2pk() {
                "Pay to Public Key (P2PK)"
            } else if output_script.is_p2pkh() {
                "Pay to Public Key Hash (P2PKH)"
            } else if output_script.is_p2sh() {
                "Pay to Script Hash (P2SH)"
            } else if output_script.is_p2wpkh() {
                "Pay to Witness Public Key Hash (P2WPKH)"
            } else if output_script.is_p2wsh() {
                "Pay to Witness Script Hash (P2WSH)"
            } else if output_script.is_p2tr() {
                "Pay to Taproot (P2TR)"
            } else {
                "unknown"
            }
        };

        TxOutResponse {
            sats_value: tx_out.value,
            btc_value: tx_out.value.to_string_in(bitcoin::Denomination::Bitcoin),
            script_pubkey: tx_out.script_pubkey.clone(),
            script: tx_out.script_pubkey.to_asm_string(),
            script_type: script_type.to_string(),
        }
    }
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
            what_is_locktime: String::from("Condition to prevent transaction from being mined until specified block height or time is \
            reached i.e if locktime is set to 620,000 then that transaction cannot be mined until that height is reached. If locktime \
            is 0, then the transaction can be included in any block"),

            input: TxInput {
                explainer: TxInExplainer {
                    txid: String::from("id of the transaction being spent. The txid + vout is called the 'outpoint'"),
                    vout: String::from("index of the specific output from the previous transaction being referenced"),
                    script_sig: String::from("script satisfying the conditions specified in the script_pubkey field from the previous outpoint \
                    referenced. This is filled for inputs spending from legacy transactions (i.e before segwit upgrade). Inputs spending from \
                    segwit outputs are empty because data will be in the witness"),
                    sequence: String::from("TODO: Can have multiple purposes"),
                    witness: String::from("only for transactions after segwit upgrade. Basically same as script_sig in that it will have the \
                    script to make the transaction valid. Difference is that data here it's not used to compute the txid")
                },
                inputs: tx.input.into_iter().map(TxInResponse::from).collect(),
            },

            output: TxOutput {
                explainer: TxOutExplainer {
                    value: String::from("amount of sats being sent"),
                    script_pubkey: String::from("script specifying the conditions that must be met to spend this output"),
                    script: String::from("script in human readable form")
                },
                output: tx.output.into_iter().map(TxOutResponse::from).collect(),
            },
        }
    }
}

// it can receive either the raw tx or the txid
pub async fn get_tx_info(tx: String) -> Result<(), String> {
    let mut tx = tx.clone();
    // get tx from esplora if txid
    if tx.len() == 64 {
        let url = format!("https://blockstream.info/api/tx/{}/hex", tx);
        tx = reqwest::get(url)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    }

    let tx: Transaction = deserialize_hex(tx.as_str()).unwrap();
    let tx = TransactionResponse::from(tx);
    let tx = serde_json::to_string_pretty(&tx).unwrap();
    println!("{}", tx);

    Ok(())
}
