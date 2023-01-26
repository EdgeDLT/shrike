use actix_web::{get, web, Responder, HttpResponse};

use crate::stat::internals;

#[allow(unused_imports)]
use crate::error::Error;
use crate::ConnectionPool;
use super::models::{
    TotalSystemFee,
    TransactionCount,
    BlockCount,
    TransferCount,
    SenderCount,
    ContractCount,
    ShrikeStats
};

#[get("/v1/stat/stats")]
async fn get_stats(pool: web::Data<ConnectionPool>) -> impl Responder {
    let conn = &pool.connection.get().unwrap();

    HttpResponse::Ok().json(ShrikeStats {
        total_blocks: internals::get_blocks_internal(conn),
        total_transactions: internals::get_transactions_internal(conn),
        total_sysfee: internals::get_sysfee_internal(conn),
        total_transfers: internals::get_transfers_internal(conn),
        total_senders: internals::get_senders_internal(conn),
        total_contracts: internals::get_contracts_internal(conn)
    })
}

#[get("/v1/stat/total_blocks")]
async fn get_total_block_count(pool: web::Data<ConnectionPool>) -> impl Responder {
    let conn = &pool.connection.get().unwrap();
    let total = internals::get_blocks_internal(conn);

    HttpResponse::Ok().json(BlockCount { total_blocks: total })
}

#[get("/v1/stat/total_transactions")]
async fn get_total_transaction_count(pool: web::Data<ConnectionPool>) -> impl Responder {
    let conn = &pool.connection.get().unwrap();
    let total = internals::get_transactions_internal(conn);

    HttpResponse::Ok().json(TransactionCount { total_transactions: total })
}

#[get("/v1/stat/total_sysfee")]
async fn get_total_sysfee(pool: web::Data<ConnectionPool>) -> impl Responder {
    let conn = &pool.connection.get().unwrap();
    let total = internals::get_sysfee_internal(conn);

    HttpResponse::Ok().json(TotalSystemFee { total_sysfee: total })
}

// Catches all transfer events (NEP-17 and NEP-11)
#[get("/v1/stat/total_transfers")]
async fn get_total_transfers(pool: web::Data<ConnectionPool>) -> impl Responder {
    let conn = &pool.connection.get().unwrap();
    let total = internals::get_transfers_internal(conn);

    HttpResponse::Ok().json(TransferCount { total_transfers: total })
}

#[get("/v1/stat/total_senders")]
async fn get_total_senders(pool: web::Data<ConnectionPool>) -> impl Responder {
    let conn = &pool.connection.get().unwrap();
    let total = internals::get_senders_internal(conn);

    HttpResponse::Ok().json(SenderCount { total_senders: total })
}

// If a transaction manages to deploy two or more contracts, this will miss the extras currently
#[get("/v1/stat/total_contracts")]
async fn get_total_contracts(pool: web::Data<ConnectionPool>) -> impl Responder {
    let conn = &pool.connection.get().unwrap();
    let total = internals::get_contracts_internal(conn);

    HttpResponse::Ok().json(ContractCount { total_contracts: total + 9 }) // fetch natives properly in future (probably when we do a contracts table)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_total_block_count)
        .service(get_total_transaction_count)
        .service(get_total_sysfee)
        .service(get_total_transfers)
        .service(get_total_senders)
        .service(get_total_contracts)
        .service(get_stats);
}
