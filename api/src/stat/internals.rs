use actix_web::web;
use once_cell::sync::Lazy;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::task;

use std::sync::RwLock;

use crate::shared::models::GAS_PRECISION;
use crate::ConnectionPool;

use super::models::{NetworkStatistics, ShrikeStats};

pub static CURRENT_NETWORK_STATISTICS: Lazy<RwLock<NetworkStatistics>> = Lazy::new(|| {
    let s = NetworkStatistics {
        total_transactions: 0,
        total_addresses: 0,
        total_contracts: 0,
        current_week_transactions: 0,
        current_week_addresses: 0,
        current_week_contracts: 0,
    };
    RwLock::new(s)
});

pub static CURRENT_STATS: Lazy<RwLock<ShrikeStats>> = Lazy::new(|| {
    let s = ShrikeStats {
        total_blocks: 0,
        total_transactions: 0,
        total_sysfee: 0.0,
        total_transfers: 0,
        total_senders: 0,
        total_contracts: 0,
    };
    RwLock::new(s)
});

pub fn get_stat_internal<T: rusqlite::types::FromSql>(
    conn: &PooledConnection<SqliteConnectionManager>,
    sql: &str,
) -> T {
    let mut stmt = conn.prepare(sql).unwrap();
    let total: Result<T, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    total.unwrap()
}

pub async fn set_stats_internal(pool: web::Data<ConnectionPool>) {
    let conn1 = pool.connection.clone().get().unwrap();

    let blocks = task::spawn_blocking(move || get_blocks_internal(&conn1))
        .await
        .unwrap();

    let current_block = CURRENT_STATS.read().unwrap().total_blocks;

    if blocks > current_block {
        let conn2 = pool.connection.clone().get().unwrap();
        let conn3 = pool.connection.clone().get().unwrap();
        let conn4 = pool.connection.clone().get().unwrap();
        let conn5 = pool.connection.clone().get().unwrap();
        let conn6 = pool.connection.clone().get().unwrap();
        let conn7 = pool.connection.clone().get().unwrap();
        let conn8 = pool.connection.clone().get().unwrap();
        let conn9 = pool.connection.clone().get().unwrap();
        let conn10 = pool.connection.clone().get().unwrap();

        let transactions = task::spawn_blocking(move || get_transactions_internal(&conn2));

        let sysfees = task::spawn_blocking(move || get_sysfee_internal(&conn3));

        let transfers = task::spawn_blocking(move || get_transfers_internal(&conn4));

        let senders = task::spawn_blocking(move || get_senders_internal(&conn5));

        let contracts = task::spawn_blocking(move || get_contracts_internal(&conn6));

        let addresses = task::spawn_blocking(move || get_addresses_internal(&conn7));

        let current_week_contracts =
            task::spawn_blocking(move || get_contracts_current_week_internal(&conn8));

        let current_week_transactions =
            task::spawn_blocking(move || get_transactions_current_week_internal(&conn9));

        let current_week_addresses =
            task::spawn_blocking(move || get_addresses_current_week_internal(&conn10));

        let results = tokio::join!(
            transactions,
            sysfees,
            transfers,
            senders,
            contracts,
            addresses,
            current_week_contracts,
            current_week_transactions,
            current_week_addresses,
        );

        let total_transactions = results.0.unwrap();
        let total_contracts = results.4.unwrap();

        {
            let mut w = CURRENT_STATS.write().unwrap();

            w.total_blocks = blocks;
            w.total_transactions = total_transactions;
            w.total_sysfee = results.1.unwrap();
            w.total_transfers = results.2.unwrap();
            w.total_senders = results.3.unwrap();
            w.total_contracts = total_contracts;
        }

        {
            let mut w = CURRENT_NETWORK_STATISTICS.write().unwrap();

            w.total_transactions = total_transactions;
            w.total_addresses = results.5.unwrap();
            w.total_contracts = total_contracts;
            w.current_week_contracts = results.6.unwrap();
            w.current_week_transactions = results.7.unwrap();
            w.current_week_addresses = results.8.unwrap();
        }
    } else {
        // println!("No cache updated needed.")
    }
    println!("Stats refreshed. Current height is {}.", blocks);
}

pub fn get_blocks_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let sql = "SELECT id FROM blocks WHERE id=(SELECT max(id) FROM blocks)";
    get_stat_internal::<u64>(conn, sql)
}

pub fn get_transactions_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let sql = "SELECT id FROM transactions WHERE id=(SELECT max(id) FROM transactions)";
    get_stat_internal::<u64>(conn, sql)
}

pub fn get_sysfee_internal(conn: &PooledConnection<SqliteConnectionManager>) -> f64 {
    let sql = "SELECT sum(sysfee) FROM transactions";
    get_stat_internal::<f64>(conn, sql) / GAS_PRECISION
}

pub fn get_transfers_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let sql = "SELECT SUM(LENGTH(notifications) - LENGTH(REPLACE(notifications, 'Transfer', ''))) / 8 FROM transactions WHERE notifications LIKE '%Transfer%'";
    get_stat_internal::<u64>(conn, sql)
}

pub fn get_senders_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let sql = "SELECT COUNT(DISTINCT sender) FROM transactions";
    get_stat_internal::<u64>(conn, sql)
}

pub fn get_contracts_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let sql = "SELECT COUNT() FROM contracts";

    let native_contracts_count = 9; // fetch natives properly in future
    get_stat_internal::<u64>(conn, &sql) + native_contracts_count
}

pub fn get_addresses_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let sql = "SELECT COUNT(DISTINCT address) FROM addresses";
    get_stat_internal::<u64>(conn, sql)
}

pub fn get_contracts_current_week_internal(
    conn: &PooledConnection<SqliteConnectionManager>,
) -> u64 {
    let sql = "SELECT COUNT(*) 
        FROM contracts 
        INNER JOIN blocks ON blocks.id = block_index 
        WHERE time >= strftime('%s', 'now', '-7 days') * 1000";
    get_stat_internal::<u64>(conn, sql)
}

pub fn get_addresses_current_week_internal(
    conn: &PooledConnection<SqliteConnectionManager>,
) -> u64 {
    let sql = "SELECT COUNT(*) 
        FROM addresses AS a
        WHERE a.block_index IN (
            SELECT b.id
            FROM blocks AS b
            WHERE b.time >= strftime('%s', 'now', '-7 days') * 1000
        )
        AND NOT EXISTS (
            SELECT 1 
            FROM addresses AS a2
            INNER JOIN blocks AS b2 ON a2.block_index = b2.id
            WHERE a2.address = a.address
            AND b2.time < strftime('%s', 'now', '-7 days') * 1000
        )";
    get_stat_internal::<u64>(conn, sql)
}

pub fn get_transactions_current_week_internal(
    conn: &PooledConnection<SqliteConnectionManager>,
) -> u64 {
    let sql = "SELECT COUNT(*) 
        FROM transactions 
        INNER JOIN blocks ON blocks.id = block_index 
        WHERE time >= strftime('%s', 'now', '-7 days') * 1000";
    get_stat_internal::<u64>(conn, sql)
}
