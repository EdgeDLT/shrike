use std::{time::SystemTime, path::Path, io, env, process::Command};
use tokio::{time::{sleep, Duration}, fs::File, io::AsyncWriteExt};
use reqwest::Client;
use clap::Parser;
use text_io::read;

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

#[cfg(target_os = "linux")]
static NEOGO_PATH: &str = "./neogo";
#[cfg(target_os = "windows")]
static NEOGO_PATH: &str = "./neogo.exe";

#[cfg(target_os = "linux")]
static NEOGO_DL: &str = "https://github.com/nspcc-dev/neo-go/releases/download/v0.101.0/neo-go-linux-amd64";
#[cfg(target_os = "windows")]
static NEOGO_DL: &str = "https://github.com/nspcc-dev/neo-go/releases/download/v0.101.0/neo-go-windows-amd64.exe";

const SLEEP_INTERVAL: u64 = 5; // in seconds, used for keep-alive mode

#[tokio::main]
async fn main() {

    let args = Args::parse();

    let start = SystemTime::now();
    println!("\nWelcome to Shrike!");

    println!("Checking for NeoGo..");
    check_neogo().await.unwrap();

    // fails if it already exists
    db::create_block_table();
    db::create_transaction_table();

    // some setup
    let index_result = db::get_last_index("blocks");
    match index_result {
        Ok(value) => {
            println!("Last stored block index: {}", value);
            value
        },
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            println!("No rows in table yet. Adding first entry...");
            db::insert_into_block_table(models::Block::genesis_block());
            0
        },
        Err(err) => panic!("Something went wrong: {:?}", err),
    };

    // Setup client and sync node
    println!("Starting node sync..");

    let client = Client::new();
    let sync_future = spawn::sync_node(NEOGO_PATH).await.unwrap();

    let sync_end = SystemTime::now();
    let sync_duration = sync_end.duration_since(start).unwrap();
    println!("Sync completed in {} ms.", sync_duration.as_millis());

    // Find the current chain height and stored height
    let mut stored_height = db::get_last_index("blocks").unwrap();
    let current_height = rpc::get_current_height(&client).await -1;
    println!("Chain height is {}.", current_height);

     // Set the batch_size too high and we'll hit a socket overload error
     // Every block takes 2 requests (getblock and getapplog)
     // Each tx takes 1 request (only need app log)
     // But there is a possibility of up to 512 tx per block
     // i.e. max 512 * batch_size requests being queued

     // Need to add sub-batching for tx requests to account for the above
     // and allow this batch size to be set higher

    let batch_size = 1000;
    let mut start_height = stored_height + 1;

    // Start our fetch and storage process
    let index_start = SystemTime::now();
    println!("Started indexing.
Start height is {}.
{} blocks to process.", start_height, current_height-start_height);

    while start_height != current_height {

        if current_height - start_height > batch_size {

            utils::sync_between(
                &client,
                start_height,
                start_height + batch_size
            ).await;

            start_height = start_height + batch_size;

        } else {

            utils::sync_between(
                &client,
                start_height,
                current_height
            ).await;

            stored_height = current_height;
            break;
        }
    }

    let index_end = SystemTime::now();
    let index_duration = index_end.duration_since(index_start).unwrap();
    println!("Indexing completed in {} ms.", index_duration.as_millis());

    if args.keep_alive == true {
        loop {
            sleep(Duration::from_secs(SLEEP_INTERVAL)).await;

            let new_height = rpc::get_current_height(&client).await -1;

            if new_height > stored_height {
                utils::sync_between(
                    &client,
                    stored_height,
                    new_height
                ).await;

                println!("Synced {} new block(s).", new_height-stored_height);
                stored_height = new_height;
            }
        }
    } else {
        println!("Killing the node now.");
    }

    // kill the node
    drop(sync_future);

}

async fn check_neogo() -> io::Result<()> {
    let path = Path::new(NEOGO_PATH);
    if !path.exists() {
        println!("NeoGo not found in directory. Install? (y/n)");
        let answer: char = read!();
        if answer != 'y' {
            panic!("User declined to install NeoGo.")
        }

        let mut file = File::create(path).await?;
        let mut response = reqwest::get(NEOGO_DL).await.unwrap();
        while let Some(chunk) = response.chunk().await.unwrap() {
            file.write_all(&chunk).await?;
        }

        if env::consts::OS == "linux" {
            println!("Updating permissions..");
            Command::new("chmod")
                .arg("+x")
                .arg(NEOGO_PATH)
                .output()
                .expect("failed to update permissions");
        }
        println!("NeoGo installed.");
    }
    Ok(())
}
