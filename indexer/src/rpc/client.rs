use anyhow::Result;
use reqwest::Client as ReqwestClient;

use crate::config::AppConfig;

use super::method::{GetApplicationLog, GetBlock, GetBlockCount, RpcMethod};
use super::models::{
    BlockAppLogResult, BlockResult, RpcRequest, RpcResponse, TransactionAppLogResult,
    TransactionResult,
};

pub struct Client {
    client: ReqwestClient,
    base_url: String,
}

impl Client {
    pub fn new(config: &AppConfig) -> Self {
        Self {
            client: ReqwestClient::new(),
            base_url: config.node_path.clone(),
        }
    }

    pub async fn send_request<T: RpcMethod, R: serde::de::DeserializeOwned>(
        &self,
        method: T,
    ) -> Result<R, reqwest::Error> {
        let request_body = RpcRequest {
            jsonrpc: "2.0".to_string(),
            id: 1,
            method: method.method_name().to_string(),
            params: method.params(),
        };
        let response: RpcResponse<R> = self
            .client
            .post(&self.base_url)
            .json(&request_body)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.result)
    }

    pub async fn get_current_height(&self) -> Result<u64> {
        let response = self.send_request(GetBlockCount).await?;
        Ok(response)
    }

    pub async fn get_block(&self, height: u64) -> Result<BlockResult> {
        let response = self
            .send_request(GetBlock {
                block_height: height,
                verbosity: 1,
            })
            .await?;
        Ok(response)
    }

    pub async fn get_application_log<T: serde::de::DeserializeOwned>(
        &self,
        hash: &str,
    ) -> Result<T> {
        let app_log = self
            .send_request(GetApplicationLog {
                hash: hash.to_string(),
            })
            .await?;
        Ok(app_log)
    }

    pub async fn fetch_full_block(&self, height: u64) -> Result<(BlockResult, BlockAppLogResult)> {
        let block = self.get_block(height).await?;
        let block_app_log: BlockAppLogResult = self.get_application_log(&block.hash).await?;

        Ok((block, block_app_log))
    }

    pub async fn fetch_full_transaction(
        &self,
        tx: TransactionResult,
    ) -> Result<(TransactionResult, TransactionAppLogResult)> {
        let tx_app_log: TransactionAppLogResult = self.get_application_log(&tx.hash).await?;

        Ok((tx, tx_app_log))
    }
}
