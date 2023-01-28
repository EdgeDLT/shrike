use actix_web::{get, web, Responder, HttpResponse};
use crate::ConnectionPool;
use crate::error::Error;
use crate::block::models::TransactionList;
use super::models::Transaction;

#[get("/v1/transaction/{hash}")]
async fn get_transaction(pool: web::Data<ConnectionPool>, path: web::Path<String>) -> impl Responder {
    let con = &pool.connection.get().unwrap();
    let hash = path.into_inner();

    let sql = "SELECT * FROM transactions WHERE hash = ?";
    let mut stmt = con.prepare(sql).unwrap();

    let result = stmt.query_row([hash], |row| {
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

    match result {
        Ok(transaction) => HttpResponse::Ok().json(transaction),
        Err(_) => HttpResponse::Ok().json(Error { error: "Transaction does not exist.".to_string() })
    }
}

#[get("/v1/transaction/sender/{address}")]
async fn get_sender_transactions(pool: web::Data<ConnectionPool>, path: web::Path<String>) -> impl Responder {
    let con = &pool.connection.get().unwrap();
    let address = path.into_inner();

    let sql = "SELECT * FROM transactions WHERE sender = ?";
    let mut stmt = con.prepare(sql).unwrap();

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
            HttpResponse::Ok().json(TransactionList { transactions: transactions })
        },
        true => {
            HttpResponse::Ok().json(Error { error: "No transactions for that sender.".to_string() })
        }
    }
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_transaction)
        .service(get_sender_transactions);
}
