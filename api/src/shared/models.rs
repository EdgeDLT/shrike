use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const GAS_PRECISION: f64 = 100000000.0;
pub const FUSDT_PRECISION: f64 = 1000000.0;

pub type Hash160 = String;
pub type Address = String;

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub index: u64,
    pub hash: String,
    pub block_index: u64,
    pub vm_state: String,
    pub size: u32,
    pub version: u8,
    pub nonce: u64,
    pub sender: String,
    pub sysfee: String,
    pub netfee: String,
    pub valid_until: u64,
    pub signers: Value,
    pub script: String,
    pub witnesses: Value,
    pub stack_result: Value,
    pub notifications: Value,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TransactionList {
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Transfer {
    pub contract: Hash160,
    pub from: Address,
    pub to: Address,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TxData {
    pub txid: String,
    pub time: u64, // unix timestamp, extra call to set it until I modify the db to store block time for transactions
    pub sysfee: f64,
    pub netfee: f64,
    pub nep17_transfers: Vec<Transfer>,
    pub nep11_transfers: Vec<Transfer>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TxDataList {
    pub address: String,
    pub as_sender: Vec<TxData>,
    pub as_participant: Vec<TxData>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Event {
    pub contract: Hash160,
    pub eventname: String,
    pub state: serde_json::Value,
}
