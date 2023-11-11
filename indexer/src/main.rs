use tokio::time::{sleep, Duration};
use anyhow::{Context, Result};
use log::{info, error, LevelFilter};

use std::time::SystemTime;

mod config;
mod logger;
mod rpc;
mod db;
mod utils;
mod spawn;

use config::AppConfig;
use db::model::Block;
use db::database::Database as LocalDatabase;
use rpc::client::Client as RpcClient;
use spawn::indexer::Indexer;

use logger::init;
use spawn::sync::sync_node;
use utils::node::check_neogo;

#[tokio::main]
async fn main() {
    init(LevelFilter::Info).expect("Failed to initialize logger");

    if let Err(e) = run().await {
        error!("Application error: {:?}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let config = AppConfig::new();

    let client = RpcClient::new(&config);
    let db = LocalDatabase::new(&config).context("Failed to initialize database")?;

    info!("Welcome to Shrike!");
    info!("Checking for NeoGo..");

    check_neogo().await.context("Failed to confirm NeoGo install")?;

    // make sure WAL journal mode is enabled
    db.set_to_wal().context("Failed to set to WAL")?;

    // fails if it already exists
    db.create_block_table().context("Failed to create block table")?;
    db.create_transaction_table().context("Failed to create transaction table")?;

    // create indexes if they don't exist
    db.create_index("idx_blocks_hash", "blocks", "hash").context("Failed to create block index")?;
    db.create_index("idx_tx_hash", "transactions", "hash").context("Failed to create txid index")?;
    db.create_index("idx_tx_senders", "transactions", "sender").context("Failed to create txsender index")?;

    // some setup
    let index_result = db.get_last_index("blocks").context("Failed to get last stored block index");
    match index_result {
        Ok(value) => {
            info!("Last stored block index: {}", value);
            value
        },
        Err(_) => {
            info!("No rows in table yet. Adding first entry...");
            db.insert_into_block_table(Block::genesis_block()).context("Failed to insert genesis block")?;
            0
        },
    };

    // spawn the node and wait for the sync to complete
    info!("Starting node sync..");
    let start = SystemTime::now();
    let (_stderr_out, handle, shutdown_tx) = sync_node(config.height_limit).await.context("Failed to sync node")?;

    let sync_end = SystemTime::now();
    let sync_duration = sync_end.duration_since(start)?;
    info!("Sync completed in {} ms.", sync_duration.as_millis());
    sleep(Duration::from_secs(2)).await;

    // Launch indexer
    let indexer = Indexer::new(client, db, config);
    indexer.run().await?;

    // send the shutdown signal to the node and wait for it to exit
    let _ = shutdown_tx.send(());
    handle.await.context("Failed to kill node")?;

    Ok(())
}