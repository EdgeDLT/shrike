use actix_web::{get, web, Responder, HttpResponse};

use crate::ConnectionPool;
use crate::error::Error;
use crate::shared::checker;
use crate::shared::models::{Transaction, TransactionList};

use super::internals;

#[get("/v1/block/{id}")]
async fn get_block(pool: web::Data<ConnectionPool>, path: web::Path<String>) -> impl Responder {
    let conn = &pool.connection.get().unwrap();
    let hash = path.into_inner();

    let result = internals::get_block_internal(conn, hash);

    match result {
        Ok(block) => HttpResponse::Ok().json(block),
        Err(_) => HttpResponse::Ok().json(Error { error: "Block does not exist.".to_string() })
    }
}

#[get("/v1/block/{id}/transactions")]
async fn get_block_transactions(pool: web::Data<ConnectionPool>, path: web::Path<String>) -> impl Responder {
    let con = &pool.connection.get().unwrap();
    let hash = path.into_inner();

    if !checker::is_neo_txid_hash(&hash) {
        return HttpResponse::Ok().json(Error { error: "Invalid block hash.".to_string() });
    }

    let sql = "SELECT * FROM transactions WHERE block_hash = ?";
    let mut stmt = con.prepare(sql).unwrap();

    let mut rows = stmt.query([hash]).unwrap();
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
            HttpResponse::Ok().json(TransactionList { transactions })
        },
        true => {
            HttpResponse::Ok().json(Error { error: "No transactions for that block.".to_string() })
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_block)
        .service(get_block_transactions);
}

