use serde::{Deserialize, Serialize};
use serde_json::Value;
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
    pub witnesses: Value,
}
