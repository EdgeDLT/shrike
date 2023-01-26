use actix_web::{get, web, Responder, HttpResponse};
use tokio::task;

use crate::stat::internals;
use crate::ConnectionPool;
use super::models::{
    // TotalSystemFee,
    // TransactionCount,
    // BlockCount,
    // TransferCount,
    // SenderCount,
    // ContractCount,
    ShrikeStats
};

/// Don't judge me. I can feel you judging me..
/// It was the only way I could figure out to make these damn calls happen in parallel!
/// The whole API is due a good refactor, I'll figure it out later...
#[get("/v1/stat/stats")]
async fn get_stats(pool: web::Data<ConnectionPool>) -> impl Responder {
    let conn1 = pool.connection.clone().get().unwrap();
    let conn2 = pool.connection.clone().get().unwrap();
    let conn3 = pool.connection.clone().get().unwrap();
    let conn4 = pool.connection.clone().get().unwrap();
    let conn5 = pool.connection.clone().get().unwrap();
    let conn6 = pool.connection.clone().get().unwrap();

    let blocks = task::spawn_blocking(move || {
        internals::get_blocks_internal(&conn1)
    });

    let transactions = task::spawn_blocking(move || {
        internals::get_transactions_internal(&conn2)
    });

    let sysfees = task::spawn_blocking(move || {
        internals::get_sysfee_internal(&conn3)
    });

    let transfers = task::spawn_blocking(move || {
        internals::get_transfers_internal(&conn4)
    });

    let senders = task::spawn_blocking(move || {
        internals::get_senders_internal(&conn5)
    });

    let contracts = task::spawn_blocking(move || {
        internals::get_contracts_internal(&conn6)
    });

    let results = tokio::join!(blocks, transactions, sysfees, transfers, senders, contracts);

    HttpResponse::Ok().json(ShrikeStats {
        total_blocks: results.0.unwrap(),
        total_transactions: results.1.unwrap(),
        total_sysfee: results.2.unwrap(),
        total_transfers: results.3.unwrap(),
        total_senders: results.4.unwrap(),
        total_contracts: results.5.unwrap()
    })
}

// #[get("/v1/stat/total_blocks")]
// async fn get_total_block_count(pool: web::Data<ConnectionPool>) -> impl Responder {
//     let conn = &pool.connection.get().unwrap();
//     let total = internals::get_blocks_internal(conn);

//     HttpResponse::Ok().json(BlockCount { total_blocks: total })
// }

// #[get("/v1/stat/total_transactions")]
// async fn get_total_transaction_count(pool: web::Data<ConnectionPool>) -> impl Responder {
//     let conn = &pool.connection.get().unwrap();
//     let total = internals::get_transactions_internal(conn);

//     HttpResponse::Ok().json(TransactionCount { total_transactions: total })
// }

// #[get("/v1/stat/total_sysfee")]
// async fn get_total_sysfee(pool: web::Data<ConnectionPool>) -> impl Responder {
//     let conn = &pool.connection.get().unwrap();
//     let total = internals::get_sysfee_internal(conn);

//     HttpResponse::Ok().json(TotalSystemFee { total_sysfee: total })
// }

// // Catches all transfer events (NEP-17 and NEP-11)
// #[get("/v1/stat/total_transfers")]
// async fn get_total_transfers(pool: web::Data<ConnectionPool>) -> impl Responder {
//     let conn = &pool.connection.get().unwrap();
//     let total = internals::get_transfers_internal(conn);

//     HttpResponse::Ok().json(TransferCount { total_transfers: total })
// }

// #[get("/v1/stat/total_senders")]
// async fn get_total_senders(pool: web::Data<ConnectionPool>) -> impl Responder {
//     let conn = &pool.connection.get().unwrap();
//     let total = internals::get_senders_internal(conn);

//     HttpResponse::Ok().json(SenderCount { total_senders: total })
// }

// // If a transaction manages to deploy two or more contracts, this will miss the extras currently
// #[get("/v1/stat/total_contracts")]
// async fn get_total_contracts(pool: web::Data<ConnectionPool>) -> impl Responder {
//     let conn = &pool.connection.get().unwrap();
//     let total = internals::get_contracts_internal(conn);

//     HttpResponse::Ok().json(ContractCount { total_contracts: total + 9 }) // fetch natives properly in future (probably when we do a contracts table)
// }

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        // .service(get_total_block_count)
        // .service(get_total_transaction_count)
        // .service(get_total_sysfee)
        // .service(get_total_transfers)
        // .service(get_total_senders)
        // .service(get_total_contracts)
        .service(get_stats);
}
