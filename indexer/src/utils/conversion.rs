use lib::neo::{
    base64_to_address, base64_to_hex, base64_to_script_hash, hex_decode, hex_to_base64,
    neo3_disassemble,
};
use serde_json::to_string;

use crate::db::model::{Address, Block, Contract, Transaction};
use crate::rpc::models::{
    BlockAppLogResult, BlockResult, TransactionAppLogResult, TransactionResult,
};

pub fn convert_block_result(r: BlockResult, a: &BlockAppLogResult) -> Block {
    let block_reward = &a.executions[1].notifications[0].state.value[2].value;
    let block_receiver = &a.executions[1].notifications[0].state.value[1].value;

    let reward_string = block_reward.clone().unwrap().as_str().unwrap().to_string();
    let reward = reward_string.parse::<u64>().unwrap();
    let reward_as_float = reward as f64 / 100_000_000_f64;

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
    a: &TransactionAppLogResult,
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
        script: base64_to_hex(&t.script),
        witnesses: to_string(&t.witnesses).unwrap(),
        stack_result: to_string(&stack).unwrap(),
        notifications: to_string(&notifs).unwrap(),
    }
}

pub fn convert_contract_result(
    script: String,
    notifications: serde_json::Value,
    block_height: u64,
) -> Vec<Contract> {
    let mut contracts = Vec::new();

    for notification in notifications.as_array().unwrap() {
        if notification["eventname"] == "Deploy"
            && notification["contract"] == "0xfffdc93764dbaddd97c48f252a53ea4643faa3fd"
        {
            let full_disassembled_script = neo3_disassemble(&hex_to_base64(&script));
            let disassembled_script: Vec<&str> = full_disassembled_script.split("\n").collect();

            let mut contract_supported_standard: String = "[]".to_string();

            if let Some(data) = disassembled_script
                .iter()
                .find(|&s| s.contains("PUSHDATA2"))
            {
                let parts: Vec<&str> = data.split_whitespace().collect();
                let metadata_hex = parts.get(1).unwrap_or(&"");
                let metadata_hex_decoded = hex_decode(metadata_hex);
                let metadata = String::from_utf8(metadata_hex_decoded).unwrap();

                if metadata.starts_with("{") {
                    let metadata_json: serde_json::Value = serde_json::from_str(&metadata).unwrap();

                    contract_supported_standard = metadata_json["supportedstandards"].to_string();
                }
            }

            let contract_hash_base64 = notification["state"]["value"][0]["value"].clone();
            let contract_script_hash =
                base64_to_script_hash(contract_hash_base64.as_str().unwrap());

            contracts.push(Contract {
                block_index: block_height,
                hash: contract_script_hash,
                contract_type: contract_supported_standard,
            });
        }
    }

    return contracts;
}

pub fn convert_address_result(notifications: serde_json::Value, block_height: u64) -> Vec<Address> {
    let mut addresses = Vec::new();

    for notification in notifications.as_array().unwrap() {
        if notification["eventname"] == "Transfer" {
            let state = notification["state"].clone();

            if let (Some(sender_type), Some(recipient_type)) = (
                state["value"][0]["type"].as_str(),
                state["value"][1]["type"].as_str(),
            ) {
                if sender_type == "ByteString" && recipient_type == "ByteString" {
                    let sender_address =
                        base64_to_address(state["value"][0]["value"].as_str().unwrap());
                    let recipient_address =
                        base64_to_address(state["value"][1]["value"].as_str().unwrap());

                    addresses.push(Address {
                        block_index: block_height,
                        address: sender_address,
                        balances: "{}".to_string(),
                    });

                    addresses.push(Address {
                        block_index: block_height,
                        address: recipient_address,
                        balances: "{}".to_string(),
                    });
                }
            }
        }
    }

    return addresses;
}
