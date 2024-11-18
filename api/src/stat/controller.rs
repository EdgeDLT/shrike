use actix_web::{get, web, HttpResponse, Responder};


use super::internals::CURRENT_STATS;
use super::internals::CURRENT_NETWORK_STATISTICS;
use super::models::ShrikeStats;
use super::models::NetworkStatistics;

// Now this path is always fast and always up to date
#[get("/v1/stat/stats")]
async fn get_stats() -> impl Responder {
    let lock = CURRENT_STATS.read().unwrap();

    HttpResponse::Ok().json(ShrikeStats {
        total_blocks: lock.total_blocks,
        total_transactions: lock.total_transactions,
        total_sysfee: lock.total_sysfee,
        total_transfers: lock.total_transfers,
        total_senders: lock.total_senders,
        total_contracts: lock.total_contracts,
    })
}

// Now this path is always fast and always up to date
#[get("/v1/stat/network-statistics")]
async fn get_network_statistics() -> impl Responder {
    let lock: std::sync::RwLockReadGuard<'_, NetworkStatistics> = CURRENT_NETWORK_STATISTICS.read().unwrap();

    HttpResponse::Ok().json(NetworkStatistics {
        total_transactions: lock.total_transactions,
        total_addresses: lock.total_addresses,
        total_contracts: lock.total_contracts,
        transactions_last_week: lock.transactions_last_week,
        addresses_last_week: lock.addresses_last_week,
        contracts_last_week: lock.contracts_last_week,
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_stats).service(get_network_statistics);
}
