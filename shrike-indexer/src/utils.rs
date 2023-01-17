use crate::rpc::{BlockResult, BlockAppLogResult, TransactionResult, TransactionAppLogResult, fetch_full_block, fetch_full_transaction};
use crate::db::{Block, Transaction, insert_blocks_transactions};
use crate::neo::{base64_to_scripthash, scripthash_to_address};
use futures::future::join_all;
use reqwest::Client;
use serde_json::to_string;

pub async fn sync_between(client: &Client, start_height: u64, end_height: u64) {

    let future_blocks = (start_height..end_height)
        .map(|i| fetch_full_block(&client, i));
    let all_blocks = join_all(future_blocks).await;

    // Have to clone to keep all_blocks unmoved for future steps
    let transactions: Vec<TransactionResult> = all_blocks
        .iter()
        .map(|(block, _)| {
            block.tx
                .iter()
                .map(|tx| {
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
                    }
                })
                .collect::<Vec<TransactionResult>>()
        })
        .flatten()
        .collect();

    let future_transactions = transactions
        .into_iter()
        .map(|tx| fetch_full_transaction(&client, tx));
    let all_transactions = join_all(future_transactions).await;

    let prepped_blocks = all_blocks
        .into_iter()
        .map(|(b, a)| convert_block_result(b, a));

    let prepped_tx = all_transactions
        .into_iter()
        .map(|(t, a)| convert_transaction_result(t, a) );

    // Dump all to DB in one step
    // It's uglier but faster and gives the tables a synced rollback point
    insert_blocks_transactions(prepped_blocks, prepped_tx).unwrap();

}

pub fn convert_block_result(r: BlockResult, a: BlockAppLogResult) -> Block {

    let block_reward = &a.executions[1].notifications[0].state.value[2].value;
    let block_receiver = &a.executions[1].notifications[0].state.value[1].value;

    let reward_string = block_reward.clone().unwrap().as_str().unwrap().to_string();
    let reward = reward_string.parse::<u64>().unwrap();
    let reward_as_float = reward as f64 / 100000000 as f64;

    let receiver = serde_json::to_string(block_receiver).unwrap();
    let stripped = &receiver[1..29];
    let script_hash = base64_to_scripthash(stripped);

    let db_block = Block {
        hash: r.hash,
        size: r.size,
        version: r.version,
        merkle_root: r.merkleroot,
        time: r.time,
        nonce: r.nonce,
        speaker: r.primary,
        next_consensus: r.nextconsensus,
        reward: reward_as_float,
        reward_receiver: scripthash_to_address(&script_hash),
        witnesses: to_string(&r.witnesses).unwrap()
    };

    db_block
}

pub fn convert_transaction_result(t: TransactionResult, a: TransactionAppLogResult) -> Transaction {

    let state = &a.executions[0].vmstate;
    let stack = &a.executions[0].stack;
    let notifs = &a.executions[0].notifications;

    let db_tx = Transaction {
        hash: t.hash,
        block_hash: t.blockhash.unwrap(),
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
    };

    db_tx
}
