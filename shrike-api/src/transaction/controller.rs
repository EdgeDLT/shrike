use actix_web::{get, web, Responder, HttpResponse};

use crate::ConnectionPool;
use crate::error::Error;
use crate::shared::checker;
use crate::shared::models::TransferList;

use super::internals;

#[get("/v1/transaction/{hash}")]
async fn get_transaction(pool: web::Data<ConnectionPool>, path: web::Path<String>) -> impl Responder {
    let conn = &pool.connection.get().unwrap();
    let hash = path.into_inner();

    if !checker::is_neo_txid_hash(&hash) {
        return HttpResponse::Ok().json(Error { error: "Invalid transaction hash.".to_string() })
    }

    let transaction = internals::get_transaction_internal(conn, hash);

    match transaction {
        Ok(tx) => HttpResponse::Ok().json(tx),
        Err(err) => HttpResponse::Ok().json(err)
    }
}

#[get("/v1/transaction/sender/{address}")]
async fn get_sender_transactions(pool: web::Data<ConnectionPool>, path: web::Path<String>) -> impl Responder {
    let conn = &pool.connection.get().unwrap();
    let address = path.into_inner();

    if !checker::is_neo_address(&address) {
        return HttpResponse::Ok().json(Error { error: "Invalid address.".to_string() })
    }

    let transactions = internals::get_sender_transactions_internal(conn, address);

    match transactions {
        Ok(txs) => HttpResponse::Ok().json(txs),
        Err(err) => HttpResponse::Ok().json(err)
    }
}

#[get("/v1/transaction/sender/{address}/transfers")]
async fn get_sender_transfers(pool: web::Data<ConnectionPool>, path: web::Path<String>) -> impl Responder {
    let conn = &pool.connection.get().unwrap();
    let address = path.into_inner();

    if !checker::is_neo_address(&address) {
        return HttpResponse::Ok().json(Error { error: "Invalid address.".to_string() })
    }

    let transfers = internals::get_sender_transfers_internal(conn, address);

    match transfers {
        Ok(txs) => HttpResponse::Ok().json(TransferList { events: txs}),
        Err(err) => HttpResponse::Ok().json(err)
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_transaction)
        .service(get_sender_transactions)
        .service(get_sender_transfers);
}
