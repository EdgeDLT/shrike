use crate::shared::models::{Transfer, Transaction, TxData};

use shrike_lib::neo::base64_to_address;

use super::models::{GAS_PRECISION, FUSDT_PRECISION};

// this approach kinda sucks, we don't get inbound transfers and the code is a mess
// needs improving later
pub fn get_transfer_events(tx: Transaction) -> TxData {
    let mut transfers = Vec::new();
    let notifications = tx.notifications.as_array().unwrap();

    for notification in notifications {

        let state = notification["state"].clone();

        if notification["eventname"] == "Transfer"
            && state["type"] == "Array"
            && state["value"][0]["type"] == "ByteString" || state["value"][0]["type"] == "Any"
            && state["value"][1]["type"] == "ByteString" || state["value"][1]["type"] == "Any"
            && state["value"][2]["type"] == "Integer" {

            let contract = notification["contract"].as_str().unwrap().to_string();
            let from;
            let to;

            if state["value"][0]["value"].is_string() {
                from = base64_to_address(state["value"][0]["value"].as_str().unwrap());
            } else {
                from = "".to_string();
            }

            if state["value"][1]["value"].is_string() {
                to = base64_to_address(state["value"][1]["value"].as_str().unwrap());
            } else {
                to = "".to_string();
            }

            let qty = state["value"][2]["value"].as_str().unwrap();

            let amount = match qty.parse::<f64>() {
                Ok(v) => {
                    if contract == "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5" {
                        v
                    } else if contract == "0xcd48b160c1bbc9d74997b803b9a7ad50a4bef020" {
                        v / FUSDT_PRECISION
                    } else {
                        v / GAS_PRECISION
                    }
                },
                Err(_) => continue
            };

            let transfer = Transfer {
                contract,
                from,
                to,
                amount // this will break on non-8 decimal contracts, will need contract table
            };

            transfers.push(transfer);
        }
    }

    let transfer_events = TxData {
        txid: tx.hash,
        block_hash: tx.block_hash,
        time: 0,
        sender: tx.sender,
        sysfee: tx.sysfee.parse::<f64>().unwrap() / GAS_PRECISION,
        netfee: tx.netfee.parse::<f64>().unwrap() / GAS_PRECISION,
        nep17_transfers: transfers,
        nep11_transfers: Vec::new()
    };
    transfer_events
}
