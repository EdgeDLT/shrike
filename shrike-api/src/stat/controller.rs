use actix_web::{get, web, Responder, HttpResponse};
use cached::proc_macro::cached;
use cached::TimedCache;
use crate::error::Error;
use crate::ConnectionPool;
use super::models::{
    TotalSystemFee,
    TransactionCount,
    BlockCount,
    InvocationCount,
    TransferCount,
    SenderCount,
    ContractCount
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

#[cached(
    type = "TimedCache<String, f64>",
    create = "{ TimedCache::with_lifespan(20) }",
    convert = r#"{ String::from("sysfee") }"#
)]
async fn get_sysfee_internal(pool: web::Data<ConnectionPool>) -> f64 {
    let con = &pool.connection.get().unwrap();

    let sql = "SELECT sum(sysfee) FROM transactions";
    let mut stmt = con.prepare(sql).unwrap();

    let res: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));
    let total = res.unwrap() as f64 / PRECISION;

    total
}

#[get("/v1/stat/total_sysfee")]
async fn get_total_sysfee(pool: web::Data<ConnectionPool>) -> impl Responder {
    let total = get_sysfee_internal(pool).await;
    HttpResponse::Ok().json(TotalSystemFee { total_sysfee: total })
}

#[cached(
    type = "TimedCache<String, u64>",
    create = "{ TimedCache::with_lifespan(20) }",
    convert = r#"{ String::from("transfers") }"#
)]
async fn get_transfers_internal(pool: web::Data<ConnectionPool>) -> u64 {
    let con = &pool.connection.get().unwrap();

    let sql = "SELECT SUM(LENGTH(notifications) - LENGTH(REPLACE(notifications, 'Transfer', ''))) / 8 FROM transactions WHERE notifications LIKE '%Transfer%'";
    let mut stmt = con.prepare(sql).unwrap();

    let total: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    total.unwrap()
}

// Catches all transfer events (NEP-17 and NEP-11)
#[get("/v1/stat/total_transfers")]
async fn get_total_transfers(pool: web::Data<ConnectionPool>) -> impl Responder {
    let total = get_transfers_internal(pool).await;
    HttpResponse::Ok().json(TransferCount { total_transfers: total })
}

#[cached(
    type = "TimedCache<String, u64>",
    create = "{ TimedCache::with_lifespan(20) }",
    convert = r#"{ String::from("senders") }"#
)]
async fn get_senders_internal(pool: web::Data<ConnectionPool>) -> u64 {
    let con = &pool.connection.get().unwrap();

    let sql = "SELECT COUNT(DISTINCT sender) FROM transactions";
    let mut stmt = con.prepare(&sql).unwrap();

    let senders: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    senders.unwrap()
}

#[get("/v1/stat/total_senders")]
async fn get_total_senders(pool: web::Data<ConnectionPool>) -> impl Responder {
    let total = get_senders_internal(pool).await;
    HttpResponse::Ok().json(SenderCount { total_senders: total })
}

#[cached(
    type = "TimedCache<String, u64>",
    create = "{ TimedCache::with_lifespan(20) }",
    convert = r#"{ String::from("contracts") }"#
)]
async fn get_contracts_internal(pool: web::Data<ConnectionPool>) -> u64 {
    let con = &pool.connection.get().unwrap();

    let deploy_event = r#"'%"contract":"0xfffdc93764dbaddd97c48f252a53ea4643faa3fd","eventname":"Deploy"%'"#;

    let sql = "SELECT COUNT() FROM transactions WHERE notifications LIKE ".to_string() + deploy_event;
    let mut stmt = con.prepare(&sql).unwrap();

    let contracts: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    contracts.unwrap()
}

// If a transaction manages to deploy two or more contracts, this will miss the extras currently
#[get("/v1/stat/total_contracts")]
async fn get_total_contracts(pool: web::Data<ConnectionPool>) -> impl Responder {
    let total = get_contracts_internal(pool).await;
    HttpResponse::Ok().json(ContractCount { total_contracts: total + 9 }) // fetch natives properly in future (probably when we do a contracts table)
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
        .service(get_total_transfers)
        .service(get_total_senders)
        .service(get_total_contracts);
}
