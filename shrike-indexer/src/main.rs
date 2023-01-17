use std::time::SystemTime;
use reqwest::Client;

mod db;
mod spawn;
mod rpc;
mod utils;
mod neo;

#[tokio::main]
async fn main() {

    let start = SystemTime::now();
    println!("\nWelcome to Shrike!");

    // fails if it already exists
    db::create_block_table();
    db::create_transaction_table();

    // some setup
    let index_result = db::get_last_block_index();
    match index_result {
        Ok(value) => {
            println!("Last stored block index: {}", value);
            value
        },
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            println!("No rows in table yet. Adding first entry...");
            db::insert_into_block_table(db::Block::genesis_block());
            0
        },
        Err(err) => panic!("Something went wrong: {:?}", err),
    };

    // Setup client and sync node
    println!("Starting node sync..");

    let client = Client::new();
    let node = spawn::NeoGo::new("./neogo.exe");
    let sync_future = node.sync_node().await.unwrap();

    let sync_end = SystemTime::now();
    let sync_duration = sync_end.duration_since(start).unwrap();
    println!("Sync completed in {} ms.", sync_duration.as_millis());

    // Find the current chain height and stored height
    let stored_height = db::get_last_block_index().unwrap();
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

            break;
        }
    }

    let index_end = SystemTime::now();
    let index_duration = index_end.duration_since(index_start).unwrap();
    println!("Indexing completed in {} ms.", index_duration.as_millis());

    // kill the node
    println!("Killing the node now.");
    drop(sync_future);

}

mod shrike_tests {

    #[test]
    fn test_base64_decode() {
        let test_string = "e6Kzuk8PbRM81V2ZgHneG4KX4zo=";
        let hex = crate::neo::base64_to_scripthash(test_string);

        assert_eq!(hex, "7ba2b3ba4f0f6d133cd55d998079de1b8297e33a");
    }

    #[test]
    fn test_script_hash_to_address() {
        let script_hash = "96949ed482e7c60aaeec691550f1b3d599146194";
        let address = crate::neo::scripthash_to_address(script_hash);

        assert_eq!(address, "NZeAarn3UMCqNsTymTMF2Pn6X7Yw3GhqDv");
    }
}


