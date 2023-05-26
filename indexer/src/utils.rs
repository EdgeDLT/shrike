use futures::future::join_all;
use reqwest::Client;
use serde_json::to_string;
use anyhow::{Context, Result};
use tokio::{time::{sleep, Duration}, fs::File, io::AsyncWriteExt};
use text_io::read;
use log::{info, warn};

use std::{path::Path, io, env, process::Command};

use lib::neo::base64_to_address;

use crate::rpc::{fetch_full_block, fetch_full_transaction, get_current_height};
use crate::db::insert_blocks_transactions;
use crate::models::{
    Block,
    Transaction,
    BlockResult,
    BlockAppLogResult,
    TransactionResult,
    TransactionAppLogResult
};

#[cfg(target_os = "linux")]
pub static NEOGO_PATH: &str = "./neogo";
#[cfg(target_os = "macos")]
pub static NEOGO_PATH: &str = "./neogo";
#[cfg(target_os = "windows")]
pub static NEOGO_PATH: &str = "./neogo.exe";

#[cfg(target_os = "linux")]
static NEOGO_DL: &str = "https://github.com/nspcc-dev/neo-go/releases/download/v0.101.0/neo-go-linux-amd64";
#[cfg(target_os = "macos")]
static NEOGO_DL: &str = "https://github.com/nspcc-dev/neo-go/releases/download/v0.101.0/neo-go-darwin-arm64";
#[cfg(target_os = "windows")]
static NEOGO_DL: &str = "https://github.com/nspcc-dev/neo-go/releases/download/v0.101.0/neo-go-windows-amd64.exe";

pub async fn sync_between(client: &Client, start_height: u64, end_height: u64) -> Result<(), anyhow::Error> {

    let future_blocks = (start_height..end_height)
        .map(|i| fetch_full_block(client, i));
    let all_blocks = join_all(future_blocks).await;

    // Have to clone to keep all_blocks unmoved for future steps
    let transactions_with_index: Vec<(TransactionResult, u64)> = all_blocks
    .iter()
    .flat_map(|(block, _)| {
        block.tx
            .iter()
            .map(move |tx| {
                (
                    TransactionResult {
                        hash: tx.hash.clone(),
                        blockhash: Some(block.hash.clone()),
                        size: tx.size,
                        version: tx.version,
                        nonce: tx.nonce,
                        sender: tx.sender.clone(),
                        sysfee: tx.sysfee.clone(),
                        netfee: tx.netfee.clone(),
                        validuntilblock: tx.validuntilblock,
                        signers: tx.signers.clone(),
                        script: tx.script.clone(),
                        witnesses: tx.witnesses.clone(),
                    },
                    block.index,
                )
            })
            .collect::<Vec<(TransactionResult, u64)>>()
    })
    .collect();

    let (transactions, block_indexes): (Vec<TransactionResult>, Vec<u64>) = transactions_with_index.into_iter().unzip();

    let future_transactions = transactions
        .into_iter()
        .map(|tx| fetch_full_transaction(client, tx));
    let all_transactions = join_all(future_transactions).await;

    let all_transactions_with_index = all_transactions
        .into_iter()
        .zip(block_indexes.into_iter());

    let prepped_blocks = all_blocks
        .into_iter()
        .map(|(b, a)| convert_block_result(b, a));

    let prepped_tx = all_transactions_with_index
        .into_iter()
        .map(|((t, a), block_index)| convert_transaction_result(t, a, block_index));

    // Dump all to DB in one step
    // It's uglier but faster and gives the tables a synced rollback point
    insert_blocks_transactions(prepped_blocks, prepped_tx)?;
    info!("Indexed {} blocks.", end_height - start_height);

    Ok(())
}

pub fn convert_block_result(r: BlockResult, a: BlockAppLogResult) -> Block {

    let block_reward = &a.executions[1].notifications[0].state.value[2].value;
    let block_receiver = &a.executions[1].notifications[0].state.value[1].value;

    let reward_string = block_reward.clone().unwrap().as_str().unwrap().to_string();
    let reward = reward_string.parse::<u64>().unwrap();
    let reward_as_float = reward as f64 / 100000000_f64;

    let receiver = serde_json::to_string(block_receiver).unwrap();
    let stripped = &receiver[1..29];
    let address = base64_to_address(stripped);

    Block {
        hash: r.hash,
        size: r.size,
        version: r.version,
        merkle_root: r.merkleroot,
        time: r.time,
        nonce: r.nonce,
        speaker: r.primary,
        next_consensus: r.nextconsensus,
        reward: reward_as_float,
        reward_receiver: address,
        witnesses: to_string(&r.witnesses).unwrap()
    }
}

pub fn convert_transaction_result(t: TransactionResult, a: TransactionAppLogResult, block_height: u64) -> Transaction {

    let state = &a.executions[0].vmstate;
    let stack = &a.executions[0].stack;
    let notifs = &a.executions[0].notifications;

    Transaction {
        hash: t.hash,
        block_index: block_height,
        vm_state: state.to_string(),
        size: t.size,
        version: t.version,
        nonce: t.nonce,
        sender: t.sender,
        sysfee: t.sysfee,
        netfee: t.netfee,
        valid_until: t.validuntilblock,
        signers: to_string(&t.signers).unwrap(),
        script: t.script,
        witnesses: to_string(&t.witnesses).unwrap(),
        stack_result: to_string(&stack).unwrap(),
        notifications: to_string(&notifs).unwrap()
    }
}

pub async fn initial_sync(client: &Client, mut start_height: u64, current_height: u64, batch_size: u64) -> Result<(), anyhow::Error> {
    while start_height != current_height {
        if current_height - start_height > batch_size {
            sync_between(client, start_height, start_height + batch_size)
                .await
                .context("Failed to synchronize block range")?;

            start_height += batch_size;
        } else {
            sync_between(client, start_height, current_height)
                .await
                .context("Failed to synchronize block range")?;

            break;
        }
    }
    Ok(())
}

pub async fn continuous_sync(client: &Client, mut start_height: u64, sleep_interval: Duration) -> Result<(), anyhow::Error> {
    loop {
        sleep(sleep_interval).await;

        let new_height = get_current_height(client)
            .await
            .context("Failed to get current height")?
            - 1;

        if new_height > start_height {
            sync_between(client, start_height, new_height)
                .await
                .context("Failed to synchronize new blocks")?;

            log::info!("Synced {} new block(s).", new_height - start_height);
            start_height = new_height;
        }
    }
}

pub async fn check_neogo() -> io::Result<()> {
    let path = Path::new(NEOGO_PATH);
    if !path.exists() {
        warn!("NeoGo not found in directory. Install? (y/n)");
        let answer: char = read!();
        if answer != 'y' {
            panic!("User declined to install NeoGo.")
        }

        let mut file = File::create(path).await?;
        let mut response = reqwest::get(NEOGO_DL).await.unwrap();
        while let Some(chunk) = response.chunk().await.unwrap() {
            file.write_all(&chunk).await?;
        }

        if env::consts::OS != "windows" {
            info!("Updating permissions..");
            Command::new("chmod")
                .arg("+x")
                .arg(NEOGO_PATH)
                .output()
                .expect("failed to update permissions");
        }
        info!("NeoGo installed.");
    }
    Ok(())
}
