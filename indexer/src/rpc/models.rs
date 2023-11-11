use serde::{Serialize, Deserialize, Serializer};

#[derive(Deserialize, Debug)]
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
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub params: Vec<NeoParam>,
    pub id: u32
}

#[derive(Deserialize, Debug)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: u32,
    pub result: T
}


#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
pub enum AppLogResult {
    BlockAppLogResult(BlockAppLogResult),
    TransactionAppLogResult(TransactionAppLogResult)
}

#[derive(Deserialize, Debug, Clone)]
pub struct BlockAppLogResult {
    pub blockhash: String,
    pub executions: Vec<Execution>
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransactionAppLogResult {
    pub txid: String,
    pub executions: Vec<Execution>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Execution {
    pub trigger: String,
    pub vmstate: String,
    pub exception: Option<String>,
    pub gasconsumed: String,
    pub stack: Vec<StateValue>,
    pub notifications: Vec<Notification>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notification {
    pub contract: String,
    pub eventname: String,
    pub state: State
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    #[serde(rename = "type")]
    pub _type: String,
    pub value: Vec<StateValue>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
