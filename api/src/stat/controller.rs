use actix_web::{get, web, Responder, HttpResponse};

use super::internals::CURRENT_STATS;
use super::models::{
    ShrikeStats
};

// Now this path is always fast and always up to date
#[get("/v1/stat/stats")]
async fn get_stats() -> impl Responder {

    let lock = CURRENT_STATS.read().unwrap();

    HttpResponse::Ok().json(ShrikeStats {
        total_blocks: lock.total_blocks,
        total_transactions: lock.total_transactions,
        total_sysfee: lock.total_sysfee,
        total_transfers: lock.total_transfers,
        total_senders:  lock.total_senders,
        total_contracts: lock.total_contracts
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_stats);
}
