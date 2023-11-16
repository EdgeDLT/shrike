use crate::shared::models::{Transaction, Transfer, TxData};

use lib::neo;

use super::models::{FUSDT_PRECISION, GAS_PRECISION};

// now supports inbound and outbound (dictated by sender field and from/to, depending on requirements)
// still needs work to support all contract decimals properly
// also it may return tons of pointless transfer data for airdrops that include the address
// not sure what to do about that right now, as we might not want to fully discount transfers
// that do not have the specified address as from/to/sender (e.g. internal transfers on DEX swaps)
pub fn get_transfer_events(tx: Transaction) -> TxData {
    let mut transfers = Vec::new();
    let notifications = tx.notifications.as_array().unwrap();

    for notification in notifications {
        let state = notification["state"].clone();

        if notification["eventname"] == "Transfer"
            && state["type"] == "Array"
            && state["value"][0]["type"] == "ByteString"
            || state["value"][0]["type"] == "Any" && state["value"][1]["type"] == "ByteString"
            || state["value"][1]["type"] == "Any" && state["value"][2]["type"] == "Integer"
        {
            let contract = notification["contract"].as_str().unwrap().to_string();

            let from = if state["value"][0]["value"].is_string() {
                neo::base64_to_address(state["value"][0]["value"].as_str().unwrap())
            } else {
                "null".to_string()
            };

            let to = if state["value"][1]["value"].is_string() {
                neo::base64_to_address(state["value"][1]["value"].as_str().unwrap())
            } else {
                "null".to_string()
            };

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
                }
                Err(_) => continue,
            };

            let transfer = Transfer {
                contract,
                from,
                to,
                amount, // this will break on non-8 decimal contracts, will need contract table
            };

            transfers.push(transfer);
        }
    }

    TxData {
        txid: tx.hash,
        time: 0,
        sysfee: tx.sysfee.parse::<f64>().unwrap() / GAS_PRECISION,
        netfee: tx.netfee.parse::<f64>().unwrap() / GAS_PRECISION,
        nep17_transfers: transfers,
        nep11_transfers: Vec::new(),
    }
}
