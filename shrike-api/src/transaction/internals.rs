use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

use crate::block::internals;
use crate::error::Error;
use crate::shared::events;
use crate::shared::models::{Transaction, TransactionList, TxData};

use shrike_lib::neo;

pub fn get_transaction_internal(conn: &PooledConnection<SqliteConnectionManager>, address: String) -> Result<Transaction, Error> {
    let sql = "SELECT * FROM transactions WHERE hash = ?";
    let mut stmt = conn.prepare(sql).unwrap();

    let transaction = stmt.query_row([address], |row| {
        Ok(Transaction {
            index: row.get(0)?,
            hash: row.get(1)?,
            block_hash: row.get(2)?,
            vm_state: row.get(3)?,
            size: row.get(4)?,
            version: row.get(5)?,
            nonce: row.get(6)?,
            sender: row.get(7)?,
            sysfee: row.get(8)?,
            netfee: row.get(9)?,
            valid_until: row.get(10)?,
            signers: row.get(11)?,
            script:row.get(12)?,
            witnesses: row.get(13)?,
            stack_result: row.get(14)?,
            notifications: row.get(15)?
        })
    });

    transaction.map_err(|_| Error { error: "Transaction does not exist.".to_string() })
}

pub fn get_sender_transactions_internal(conn: &PooledConnection<SqliteConnectionManager>, address: String) -> Result<TransactionList, Error> {
    let sql = "SELECT * FROM transactions WHERE sender = ?";
    let mut stmt = conn.prepare(sql).unwrap();

    let mut rows = stmt.query([address]).unwrap();
    let mut transactions = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        transactions.push(Transaction {
            index: row.get(0).unwrap(),
            hash: row.get(1).unwrap(),
            block_hash: row.get(2).unwrap(),
            vm_state: row.get(3).unwrap(),
            size: row.get(4).unwrap(),
            version: row.get(5).unwrap(),
            nonce: row.get(6).unwrap(),
            sender: row.get(7).unwrap(),
            sysfee: row.get(8).unwrap(),
            netfee: row.get(9).unwrap(),
            valid_until: row.get(10).unwrap(),
            signers: row.get(11).unwrap(),
            script: row.get(12).unwrap(),
            witnesses: row.get(13).unwrap(),
            stack_result: row.get(14).unwrap(),
            notifications: row.get(15).unwrap()
        })
    }

    match transactions.is_empty() {
        false => {
            Ok(TransactionList { transactions })
        },
        true => {
            Err(Error { error: "No transactions for that sender.".to_string() })
        }
    }
}

pub fn get_address_transfers_internal(conn: &PooledConnection<SqliteConnectionManager>, address: String) -> Result<Vec<TxData>, Error> {

    let base64 = neo::address_to_base64(&address);
    let sql = "SELECT * FROM transactions WHERE notifications LIKE ?";
    let mut stmt = conn.prepare(sql).unwrap();

    let mut rows = stmt.query([format!("%{}%", base64)]).unwrap();
    let mut transactions = Vec::new();

    while let Some(row) = rows.next().unwrap() {
        transactions.push(Transaction {
            index: row.get(0).unwrap(),
            hash: row.get(1).unwrap(),
            block_hash: row.get(2).unwrap(),
            vm_state: row.get(3).unwrap(),
            size: row.get(4).unwrap(),
            version: row.get(5).unwrap(),
            nonce: row.get(6).unwrap(),
            sender: row.get(7).unwrap(),
            sysfee: row.get(8).unwrap(),
            netfee: row.get(9).unwrap(),
            valid_until: row.get(10).unwrap(),
            signers: row.get(11).unwrap(),
            script: row.get(12).unwrap(),
            witnesses: row.get(13).unwrap(),
            stack_result: row.get(14).unwrap(),
            notifications: row.get(15).unwrap()
        })
    }

    let mut transfers: Vec<TxData> = Vec::new();
    for transaction in transactions {
        let block_time = internals::get_block_time(conn, transaction.block_hash.clone()).unwrap();
        let mut tx_data = events::get_transfer_events(transaction);
        tx_data.time = block_time;
        transfers.push(tx_data);
    }

    if transfers.is_empty() {
        Err(Error { error: "No transfers for that sender.".to_string() })
    } else {
        Ok(transfers)
    }
}
