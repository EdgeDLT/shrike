use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// NeoGo default
const NODE_PATH: &str = "http://localhost:10332";

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

#[derive(Serialize, Deserialize, Debug)]
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

impl Clone for TransactionResult {
    fn clone(&self) -> Self {
        TransactionResult {
            hash: self.hash.clone(),
            blockhash: self.blockhash.clone(),
            size: self.size,
            version: self.version,
            nonce: self.nonce,
            sender: self.sender.clone(),
            sysfee: self.sysfee.clone(),
            netfee: self.netfee.clone(),
            validuntilblock: self.validuntilblock,
            signers: self.signers.clone(),
            script: self.script.clone(),
            witnesses: self.witnesses.clone(),
        }
    }
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Witness {
    pub invocation: String,
    pub verification: String
}

impl Clone for Witness {
    fn clone(&self) -> Self {
        Witness {
            invocation: self.invocation.clone(),
            verification: self.verification.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Signer {
    pub account: String,
    pub scopes: String,
    pub allowedcontracts: Option<Vec<String>>
}

impl Clone for Signer {
    fn clone(&self) -> Self {
        Signer {
            account: self.account.clone(),
            scopes: self.scopes.clone(),
            allowedcontracts: self.allowedcontracts.clone(),
        }
    }
}

pub async fn neo_fetch(client: &Client, method: NeoMethod, arg: NeoParam) -> NeoResponse {

    match method {
        NeoMethod::GetBlock => {
            let args = vec![arg, NeoParam::Integer(1)];
            let request = RpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "getblock".to_string(),
                params: args,
                id: 1,
            };

            let response = client.post(NODE_PATH)
                .json(&request)
                .send()
                .await
                .unwrap()
                .json::<BlockResponse>()
                .await;

            NeoResponse::BlockResponse(response.unwrap())
        },
        NeoMethod::GetApplicationLog => {
            let mut args = Vec::new();
            args.push(arg);
            let request = RpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "getapplicationlog".to_string(),
                params: args,
                id: 1
            };

            let response = client.post(NODE_PATH)
                .json(&request)
                .send()
                .await
                .unwrap()
                .json::<ApplicationLogResponse>()
                .await;

            NeoResponse::ApplicationLogResponse(response.unwrap())
        },
        NeoMethod::GetBlockCount => {
            let args: Vec<NeoParam> = Vec::new();
            let request = RpcRequest {
                jsonrpc: "2.0".to_string(),
                method: "getblockcount".to_string(),
                params: args,
                id: 1
            };

            let response = client.post(NODE_PATH)
                .json(&request)
                .send()
                .await
                .unwrap()
                .json::<BlockCountResponse>()
                .await;

            NeoResponse::BlockCountResponse(response.unwrap())
        }
    }
}

pub async fn get_current_height(client: &Client) -> u64 {
    let height_response = neo_fetch(
        &client,
        NeoMethod::GetBlockCount,
        NeoParam::Integer(0))
        .await;
    if let NeoResponse::BlockCountResponse(node_height) = height_response {
        node_height.result
    } else {
        panic!("Failed to get chain height.")
    }
}

pub async fn fetch_full_block(client: &Client, i: u64) -> (BlockResult, BlockAppLogResult) {
    let block_response: NeoResponse = neo_fetch(
        &client,
        NeoMethod::GetBlock,
        NeoParam::Integer(i))
    .await;

    let block;
    if let NeoResponse::BlockResponse(b) = block_response {
        block = b;
    } else {
        panic!("Couldn't convert block response.")
    };

    let app_log_response = neo_fetch(
        &client,
        NeoMethod::GetApplicationLog,
        NeoParam::String(block.result.hash.to_string()))
        .await;

    let app_log;
    if let NeoResponse::ApplicationLogResponse(a) = app_log_response {
        if let AppLogResult::BlockAppLogResult(r) = a.result {
            app_log = r;
        } else {
            panic!("Couldn't convert app log result to block variant.")
        }
    } else {
        panic!("Couldn't convert app log response.");
    }

    (block.result, app_log)
}

pub async fn fetch_full_transaction(client: &Client, tx: TransactionResult) -> (TransactionResult, TransactionAppLogResult) {
    let hash = tx.hash.clone();
    let app_log_response = neo_fetch(
        &client,
        NeoMethod::GetApplicationLog,
        NeoParam::String(hash))
        .await;

        let app_log;
        if let NeoResponse::ApplicationLogResponse(a) = app_log_response {
            if let AppLogResult::TransactionAppLogResult(r) = a.result {
                app_log = r;
            } else {
                panic!("Couldn't convert app log result to tx variant.")
            }
        } else {
            panic!("Couldn't convert app log response.");
        }

        (tx, app_log)
}
