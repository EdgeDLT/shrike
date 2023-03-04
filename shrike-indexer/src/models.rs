// DB
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct Transaction {
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
    pub signers: String,
    pub script: String,
    pub witnesses: String,
    pub stack_result: String,
    pub notifications: String
}

#[derive(Debug)]
pub struct Block {
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
    pub witnesses: String
}

impl Block {
    pub fn genesis_block() -> Block {
        Block {
            hash: String::from("0x1f4d1defa46faa5e7b9b8d3f79a06bec777d7c26c4aa5f6f5899a291daa87c15"),
            size: 114,
            version: 0,
            merkle_root: String::from("0x0000000000000000000000000000000000000000000000000000000000000000"),
            time: 1468595301000,
            nonce: String::from("000000007C2BAC1D"),
            speaker: 0,
            next_consensus: String::from("NSiVJYZej4XsxG5CUpdwn7VRQk8iiiDMPM"),
            reward: 0.5,
            reward_receiver: String::from("NZeAarn3UMCqNsTymTMF2Pn6X7Yw3GhqDv"),
            witnesses: r#"[{"invocation":"DEAq7W/jUhpMon1t9muqXKfBvNyGwLfFFM1vAxrMKvUl6MqK+LL/lJAJP9uAk/cberIWWhSsdcxUtltkBLemg/VuDECQZGuvP93JlZga2ml8cnbe5cNiGgO0EMrbGYyzvgr8calP5SwMNPSYms10gIHxlsuXDU++EQpZu/vKxfHoxdC5DEDgsA3POVZdfN+i5+ekvtsaIvif42n0GC+dZi3Rp37ETmt4NtkoK2I2UXi+WIjm5yXLJsPhAvEV6cJSrvqBdsQBDEDTS6NU+kB+tgeBe9lWv+6y0L2qcUBIaUxiTCaNWZtLPghQICBvjDz1/9ttJRXG3I5N9CFDjjLKCpdIY842HW4/DEC+wlWjkCzVqzKslvpCKZbEPUGIf87CFAD88xqzl26m/TpTUcT0+D5oI2bVzAk0mcdBTPnyjcNbv17BFmr63+09","verification":"FQwhAkhv0VcCxEkKJnAxEqXMHQkj/Wl6M0Br1aHADgATsJpwDCECTHt/tsMQ/M8bozsIJRnYKWTqk4aNZ2Zi1KWa1UjfDn0MIQKq7DhHD2qtAELG6HfP2Ah9Jnaw9Rb93TYoAbm9OTY5ngwhA7IJ/U9TpxcOpERODLCmu2pTwr0BaSaYnPhfmw+6F6cMDCEDuNnVdx2PUTqghpucyNUJhkA7eMbaNokGOMPUalrc4EoMIQLKDidpe5wkj28W4IX9AGHib0TahbWO6DXBEMql7DulVAwhAt9I9g6PPgHEj/QLm38TENeosqGTGIvv4cLj33QOiVCTF0Ge0Nw6"}]"#.to_string()
        }
    }
}

// RPC
pub enum NeoMethod {
    GetBlock,
    GetApplicationLog,
    GetBlockCount
}

#[derive(Debug)]
pub enum NeoResponse {
    BlockResponse(BlockResponse),
    ApplicationLogResponse(ApplicationLogResponse),
    BlockCountResponse(BlockCountResponse)
}

#[derive(Debug)]
pub enum NeoParam {
    String(String),
    Integer(u64)
}

impl Serialize for NeoParam {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            NeoParam::String(ref value) => serializer.serialize_str(value),
            NeoParam::Integer(value) => serializer.serialize_u64(value)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RpcRequest<NeoParam> {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<NeoParam>,
    pub id: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockResponse {
    pub jsonrpc: String,
    pub id: u32,
    pub result: BlockResult
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionResponse {
    pub jsonrpc: String,
    pub id: u32,
    pub result: TransactionResult
}

#[derive(Serialize, Debug)]
pub struct ApplicationLogResponse {
    pub jsonrpc: String,
    pub id: u32,
    pub result: AppLogResult
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockCountResponse {
    pub jsonrpc: String,
    pub id: u32,
    pub result: u64
}

impl<'de> Deserialize<'de> for ApplicationLogResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            jsonrpc: String,
            id: u32,
            result: serde_json::Value,
        }

        let helper = Helper::deserialize(deserializer)?;

        let result = if let Some(_) = helper.result.get("blockhash") {
            AppLogResult::BlockAppLogResult(Deserialize::deserialize(helper.result).unwrap())
        } else {
            AppLogResult::TransactionAppLogResult(Deserialize::deserialize(helper.result).unwrap())
        };

        Ok(ApplicationLogResponse {
            jsonrpc: helper.jsonrpc,
            id: helper.id,
            result,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockResult {
    pub hash: String,
    pub size: u32,
    pub version: u8,
    pub merkleroot: String,
    pub time: u64,
    pub nonce: String,
    pub index: u64,
    pub primary: u8,
    pub nextconsensus: String,
    pub witnesses: Vec<Witness>,
    pub tx: Vec<TransactionResult>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionResult {
    pub hash: String,
    pub blockhash: Option<String>,
    pub size: u32,
    pub version: u8,
    pub nonce: u64,
    pub sender: String,
    pub sysfee: String,
    pub netfee: String,
    pub validuntilblock: u64,
    pub signers: Vec<Signer>,
    pub script: String,
    pub witnesses: Vec<Witness>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AppLogResult {
    BlockAppLogResult(BlockAppLogResult),
    TransactionAppLogResult(TransactionAppLogResult)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockAppLogResult {
    pub blockhash: String,
    pub executions: Vec<Execution>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionAppLogResult {
    pub txid: String,
    pub executions: Vec<Execution>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Execution {
    pub trigger: String,
    pub vmstate: String,
    pub exception: Option<String>,
    pub gasconsumed: String,
    pub stack: Vec<StateValue>,
    pub notifications: Vec<Notification>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notification {
    pub contract: String,
    pub eventname: String,
    pub state: State
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    #[serde(rename = "type")]
    pub _type: String,
    pub value: Vec<StateValue>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StateValue {
    #[serde(rename = "type")]
    pub _type: String,
    pub value: Option<serde_json::Value>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Witness {
    pub invocation: String,
    pub verification: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Signer {
    pub account: String,
    pub scopes: String,
    pub allowedcontracts: Option<Vec<String>>
}
