use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::transaction::models::Transaction;

#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    pub index: u64,
    pub hash: String,
    pub size: u32,
    pub version: u8,
    pub merkle_root: String,
    pub time: u64,
    pub nonce: String,
    pub speaker: u8,
    pub next_consensus: String,
    pub reward: f64,
    pub reward_receiver: String,
    pub witnesses: Value
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BlockTransactions {
    pub transactions: Vec<Transaction>,
}
