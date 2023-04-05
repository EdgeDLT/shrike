use tokio::time::{sleep, Duration};
use reqwest::Client;
use clap::Parser;
use anyhow::{Context, Result};
use log::{info, error, LevelFilter};

use std::time::SystemTime;

mod db;
mod spawn;
mod rpc;
mod utils;
mod models;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Keeps Indexer alive and syncing new blocks", default_value_t = false)]
    keep_alive: bool
}

const SLEEP_INTERVAL: u64 = 5; // in seconds, used for keep-alive mode

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .format_timestamp(None)
        .init();

    if let Err(e) = run().await {
        error!("Application error: {:?}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let args = Args::parse();
    let start = SystemTime::now();

    info!("Welcome to Shrike!");
    info!("Checking for NeoGo..");
    utils::check_neogo().await.context("Failed to confirm NeoGo install")?;

    // fails if it already exists
    db::create_block_table().context("Failed to create block table")?;
    db::create_transaction_table().context("Failed to create transaction table")?;

    // some setup
    let index_result = db::get_last_index("blocks").context("Failed to get last stored block index");
    match index_result {
        Ok(value) => {
            info!("Last stored block index: {}", value);
            value
        },
        Err(_) => {
            info!("No rows in table yet. Adding first entry...");
            db::insert_into_block_table(models::Block::genesis_block()).context("Failed to insert genesis block")?;
            0
        },
    };

    let client = Client::new();

    // spawn the node and wait for the sync to complete
    info!("Starting node sync..");
    let (_stderr_out, handle, shutdown_tx) = spawn::sync_node(utils::NEOGO_PATH).await.context("Failed to sync node")?;

    let sync_end = SystemTime::now();
    let sync_duration = sync_end.duration_since(start)?;
    info!("Sync completed in {} ms.", sync_duration.as_millis());

    // Add a delay before attempting to connect
    sleep(Duration::from_secs(2)).await;

    // Find the current chain height and stored height
    let stored_height = db::get_last_index("blocks").context("Failed to get latest stored index")?;
    let current_height = rpc::get_current_height(&client).await.context("Failed to get current height")? - 1;
    info!("Chain height is {}.", current_height);

    // Set the batch_size too high and we'll hit a socket overload error
    // Every block takes 2 requests (getblock and getapplog)
    // Each tx takes 1 request (only need app log)
    // But there is a possibility of up to 512 tx per block
    // i.e. max 512 * batch_size requests being queued

     let batch_size = 1000;
     let start_height = stored_height + 1;

     // Start our fetch and storage process
     let index_start = SystemTime::now();
     info!("Started indexing.");
     info!("Start height is {}. {} blocks to process.", start_height, current_height-start_height);

     utils::initial_sync(&client, start_height, current_height, batch_size).await?;

     let index_end = SystemTime::now();
     let index_duration = index_end.duration_since(index_start)?;
     let new_stored_height = db::get_last_index("blocks").context("Failed to get latest stored index")?;
     info!("Indexing completed in {} ms.", index_duration.as_millis());
     info!("New stored height is {}.", new_stored_height);

     // if we're in keep-alive mode, keep the node running and sync new blocks
    if args.keep_alive {
        let start_height = new_stored_height + 1;
        utils::continuous_sync(&client, start_height, Duration::from_secs(SLEEP_INTERVAL)).await?;
    } else {
        // send the shutdown signal to the node and wait for it to exit
        let _ = shutdown_tx.send(());
        handle.await.context("Failed to kill node")?;
    }

    Ok(())
}
