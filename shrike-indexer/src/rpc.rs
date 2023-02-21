use reqwest::Client;

use crate::models::{
    BlockResponse,
    ApplicationLogResponse,
    BlockCountResponse,
    BlockResult,
    BlockAppLogResult,
    AppLogResult,
    NeoMethod,
    NeoParam,
    NeoResponse,
    RpcRequest,
    TransactionResult,
    TransactionAppLogResult
};

// NeoGo default
const NODE_PATH: &str = "http://localhost:10332";

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
