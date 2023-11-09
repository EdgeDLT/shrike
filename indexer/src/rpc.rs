use reqwest::{Client, Response};

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

pub async fn get_response(client: &Client, request: RpcRequest<NeoParam>) -> Result<Response, anyhow::Error> {
    let result = client.post(NODE_PATH)
        .json(&request)
        .send()
        .await?;

    Ok(result)
}

pub fn build_request(method: &str, args: Vec<NeoParam>) -> RpcRequest<NeoParam> {
    RpcRequest {
        jsonrpc: "2.0".to_string(),
        method: method.to_string(),
        params: args,
        id: 1
    }
}

pub async fn parse_response<NeoResponse: for<'de> serde::Deserialize<'de>>(response: Response) -> NeoResponse {
    response.json::<NeoResponse>().await.unwrap()
}

pub async fn neo_fetch(client: &Client, method: NeoMethod, arg: NeoParam) -> NeoResponse {
    match method {
        NeoMethod::Block => {
            let args = vec![arg, NeoParam::Integer(1)];
            let request = build_request("getblock", args);
            let response = get_response(client, request).await.unwrap();
            let json = parse_response::<BlockResponse>(response).await;

            NeoResponse::Block(json)
        },
        NeoMethod::ApplicationLog => {
            let args = vec![arg];
            let request = build_request("getapplicationlog", args);
            let response = get_response(client, request).await.unwrap();
            let json = parse_response::<ApplicationLogResponse>(response).await;

            NeoResponse::ApplicationLog(json)
        },
        NeoMethod::BlockCount => {
            let args: Vec<NeoParam> = Vec::new();
            let request = build_request("getblockcount", args);
            let response = get_response(client, request).await.unwrap();
            let json = parse_response::<BlockCountResponse>(response).await;

            NeoResponse::BlockCount(json)
        }
    }
}

pub async fn get_current_height(client: &Client) -> Result<u64, anyhow::Error> {

    let height_response = neo_fetch(
        client,
        NeoMethod::BlockCount,
        NeoParam::Integer(0))
        .await;
    if let NeoResponse::BlockCount(node_height) = height_response {
        Ok(node_height.result)
    } else {
        Err(anyhow::anyhow!("Failed to get chain height."))
    }
}

pub async fn fetch_full_block(client: &Client, i: u64) -> (BlockResult, BlockAppLogResult) {
    let block_response: NeoResponse = neo_fetch(
        client,
        NeoMethod::Block,
        NeoParam::Integer(i))
    .await;

    let block;
    if let NeoResponse::Block(b) = block_response {
        block = b;
    } else {
        panic!("Couldn't convert block response.")
    };

    let app_log_response = neo_fetch(
        client,
        NeoMethod::ApplicationLog,
        NeoParam::String(block.result.hash.to_string()))
        .await;

    let app_log;
    if let NeoResponse::ApplicationLog(a) = app_log_response {
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
        client,
        NeoMethod::ApplicationLog,
        NeoParam::String(hash))
        .await;

        let app_log;
        if let NeoResponse::ApplicationLog(a) = app_log_response {
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
