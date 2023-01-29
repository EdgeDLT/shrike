use crate::shared::models::{Transfer, Transaction, TransferDetails};

use super::models::NEO_PRECISION;

pub fn get_all_nep17_transfers(tx: Transaction) -> TransferDetails {
    let mut transfers = Vec::new();
    let notifications = tx.notifications.as_array().unwrap();

    for notification in notifications {

        let state = notification["state"].clone();

        if notification["eventname"] == "Transfer"
            && state["type"] == "Array"
            && state["value"][0]["type"] == "ByteString"
            && state["value"][1]["type"] == "ByteString"
            && state["value"][2]["type"] == "Integer" {

            let contract = notification["contract"].as_str().unwrap();
            let from = state["value"][0]["value"].as_str();
            let to = state["value"][1]["value"].as_str().unwrap();
            let amount = state["value"][2]["value"].as_str().unwrap();

            let transfer = Transfer {
                contract: contract.to_string(),
                from: from.unwrap().to_string(),
                to: to.to_string(),
                amount: amount.parse::<f64>().unwrap() / NEO_PRECISION // this will break on non-8 decimal contracts, will need contract table
            };

            transfers.push(transfer);
        }
    }

    let transfer_events = TransferDetails {
        txid: tx.hash,
        block_hash: tx.block_hash,
        sender: tx.sender,
        sysfee: tx.sysfee,
        netfee: tx.netfee,
        transfers
    };
    transfer_events
}
