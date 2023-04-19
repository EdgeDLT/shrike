use actix_web::{get, web, Responder, HttpResponse};

use crate::ConnectionPool;
use crate::error::Error;
use crate::shared::models::TransactionList;

use super::internals;

#[get("/v1/block/{id}")]
async fn get_block(pool: web::Data<ConnectionPool>, path: web::Path<String>) -> impl Responder {
    let conn = &pool.connection.get().unwrap();
    let id = path.into_inner();

    let result = internals::get_block_internal(conn, id);

    match result {
        Ok(block) => HttpResponse::Ok().json(block),
        Err(_) => HttpResponse::Ok().json(Error { error: "Block does not exist.".to_string() })
    }
}

#[get("/v1/block/{id}/transactions")]
async fn get_block_transactions(pool: web::Data<ConnectionPool>, path: web::Path<String>) -> impl Responder {
    let conn = &pool.connection.get().unwrap();
    let id = path.into_inner();

    let transactions = internals::get_block_transactions_internal(conn, id).unwrap();

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

