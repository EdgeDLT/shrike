use actix_web::{get, web, HttpResponse, Responder};

use super::internals::CURRENT_NETWORK_STATISTICS;
use super::internals::CURRENT_STATS;
use super::models::NetworkStatistics;
use super::models::ShrikeStats;

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

#[get("/v1/stat/network-statistics")]
async fn get_network_statistics() -> impl Responder {
    let lock: std::sync::RwLockReadGuard<'_, NetworkStatistics> =
        CURRENT_NETWORK_STATISTICS.read().unwrap();

    HttpResponse::Ok().json(NetworkStatistics {
        total_transactions: lock.total_transactions,
        total_addresses: lock.total_addresses,
        total_contracts: lock.total_contracts,
        current_week_transactions: lock.current_week_transactions,
        current_week_addresses: lock.current_week_addresses,
        current_week_contracts: lock.current_week_contracts,
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_stats).service(get_network_statistics);
}
