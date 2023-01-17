use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub index: u64,
    pub hash: String,
    pub block_hash: String,
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
    pub notifications: Value
}
