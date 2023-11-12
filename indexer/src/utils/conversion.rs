use lib::neo::base64_to_address;
use serde_json::to_string;

use crate::db::model::{Block, Transaction};
use crate::rpc::models::{
    BlockAppLogResult, BlockResult, TransactionAppLogResult, TransactionResult,
};

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
        witnesses: to_string(&r.witnesses).unwrap(),
    }
}

pub fn convert_transaction_result(
    t: TransactionResult,
    a: TransactionAppLogResult,
    block_height: u64,
) -> Transaction {
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
        notifications: to_string(&notifs).unwrap(),
    }
}
