use actix_web::{get, web, Responder, HttpResponse};
use crate::error::Error;
use crate::ConnectionPool;
use super::models::{
    TotalSystemFee,
    TransactionCount,
    BlockCount,
    InvocationCount,
    ContractCount,
    TransferCount,
    SenderCount,
    DeployCount
};

const PRECISION: f64 = 100000000.0;

#[get("/v1/stat/total_blocks")]
async fn get_total_block_count(pool: web::Data<ConnectionPool>) -> impl Responder {
    let con = &pool.connection.get().unwrap();

    let sql = "SELECT id FROM blocks WHERE id=(SELECT max(id) FROM blocks)";
    let mut stmt = con.prepare(sql).unwrap();

    let total: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    HttpResponse::Ok().json(BlockCount { total_blocks: total.unwrap() })
}

#[get("/v1/stat/total_transactions")]
async fn get_total_transaction_count(pool: web::Data<ConnectionPool>) -> impl Responder {
    let con = &pool.connection.get().unwrap();

    let sql = "SELECT id FROM transactions WHERE id=(SELECT max(id) FROM transactions)";
    let mut stmt = con.prepare(sql).unwrap();

    let total: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    HttpResponse::Ok().json(TransactionCount { total_transactions: total.unwrap() })
}

#[get("/v1/stat/total_sysfee")]
async fn get_total_sysfee(pool: web::Data<ConnectionPool>) -> impl Responder {
    let con = &pool.connection.get().unwrap();

    let sql = "SELECT sum(sysfee) FROM transactions";
    let mut stmt = con.prepare(sql).unwrap();

    let res: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));
    let total = res.unwrap() as f64 / PRECISION;

    HttpResponse::Ok().json(TotalSystemFee { total_sysfee: total as f64 })
}

#[get("/v1/stat/total_contracts")]
async fn get_total_contracts(pool: web::Data<ConnectionPool>) -> impl Responder {
    let con = &pool.connection.get().unwrap();

    let sql = "SELECT COUNT() FROM transactions WHERE notifications LIKE '%Deploy%'";
    let mut stmt = con.prepare(sql).unwrap();

    let res: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    HttpResponse::Ok().json(ContractCount { total_contracts: res.unwrap() })
}

// A transaction with multiple transfers will only be counted as one currently
#[get("/v1/stat/total_transfers")]
async fn get_total_transfers(pool: web::Data<ConnectionPool>) -> impl Responder {
    let con = &pool.connection.get().unwrap();

    let sql = "SELECT COUNT() FROM transactions WHERE notifications LIKE '%Transfer%'";
    let mut stmt = con.prepare(sql).unwrap();

    let res: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    HttpResponse::Ok().json(TransferCount { total_transfers: res.unwrap() })
}

#[get("/v1/stat/total_senders")]
async fn get_total_senders(pool: web::Data<ConnectionPool>) -> impl Responder {
    let con = &pool.connection.get().unwrap();

    let sql = "SELECT COUNT(DISTINCT sender) FROM transactions";
    let mut stmt = con.prepare(sql).unwrap();

    let res: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    HttpResponse::Ok().json(SenderCount { total_senders: res.unwrap() })
}

// If a transaction manages to deploy two or more contracts, this will miss the extras currently
#[get("/v1/stat/total_deploys")]
async fn get_total_deploys(pool: web::Data<ConnectionPool>) -> impl Responder {
    let con = &pool.connection.get().unwrap();

    let sql = "SELECT COUNT() FROM transactions WHERE notifications LIKE '%Deploy%'";
    let mut stmt = con.prepare(sql).unwrap();

    let total: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    match total {
        Ok(deploys) => {
            if deploys == 0 {
                HttpResponse::Ok().json(Error { error: "No results found for that contract hash.".to_string() })
            } else {
                HttpResponse::Ok().json(DeployCount { total_deploys: deploys })
            }
        },
        Err(_) => HttpResponse::Ok().json(Error { error: "Unknown error occurred.".to_string() })
    }
}

// If a transaction manages to invoke a contract more than once, this will miss the extras currently
#[get("/v1/stat/total_invocations/{hash}")]
async fn get_total_invocations(pool: web::Data<ConnectionPool>, path: web::Path<String>) -> impl Responder {
    let con = &pool.connection.get().unwrap();
    let hash = path.into_inner();

    let sql = "SELECT COUNT() FROM transactions WHERE notifications LIKE ?";
    let mut stmt = con.prepare(sql).unwrap();

    let total: Result<u64, rusqlite::Error> = stmt.query_row([format!("%{}%", hash)], |row| row.get(0));

    match total {
        Ok(invocations) => {
            if invocations == 0 {
                HttpResponse::Ok().json(Error { error: "No results found for that contract hash.".to_string() })
            } else {
                HttpResponse::Ok().json(InvocationCount { total_invocations: invocations })
            }
        },
        Err(_) => HttpResponse::Ok().json(Error { error: "Unknown error occurred.".to_string() })
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_total_block_count)
        .service(get_total_transaction_count)
        .service(get_total_sysfee)
        .service(get_total_invocations)
        .service(get_total_contracts)
        .service(get_total_transfers)
        .service(get_total_senders)
        .service(get_total_deploys);
}
