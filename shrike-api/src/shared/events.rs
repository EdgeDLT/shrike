use crate::shared::models::{Transfer, Transaction, TxData};

use super::models::NEO_PRECISION;

pub fn get_transfer_events(tx: Transaction) -> TxData {
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
            let from = state["value"][0]["value"].as_str().unwrap();
            let to = state["value"][1]["value"].as_str().unwrap();
            let amount = state["value"][2]["value"].as_str().unwrap();

            let transfer = Transfer {
                contract: contract.to_string(),
                from: from.to_string(),
                to: to.to_string(),
                amount: amount.parse::<f64>().unwrap() / NEO_PRECISION // this will break on non-8 decimal contracts, will need contract table
            };

            transfers.push(transfer);
        }
    }

    let transfer_events = TxData {
        txid: tx.hash,
        block_hash: tx.block_hash,
        time: 0,
        sender: tx.sender,
        sysfee: tx.sysfee.parse::<f64>().unwrap() / NEO_PRECISION,
        netfee: tx.netfee.parse::<f64>().unwrap() / NEO_PRECISION,
        nep17_transfers: transfers,
        nep11_transfers: Vec::new()
    };
    transfer_events
}
