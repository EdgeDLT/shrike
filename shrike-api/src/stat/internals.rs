use actix_web::web;
use tokio::task;
use once_cell::sync::Lazy;

use std::sync::RwLock;

use crate::ConnectionPool;
use super::models::ShrikeStats;

use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

pub static CURRENT_STATS: Lazy<RwLock<ShrikeStats>> = Lazy::new(|| {
    let s = ShrikeStats {
        total_blocks: 0,
        total_transactions: 0,
        total_sysfee: 0.0,
        total_transfers: 0,
        total_senders: 0,
        total_contracts:0,
    };
    RwLock::new(s)
});

const PRECISION: f64 = 100000000.0;

// Should be possible to make this generic but the way to do it in this context escapes me currently
pub fn get_unsigned_stat_internal(conn: &PooledConnection<SqliteConnectionManager>, sql: &str) -> u64 {
    let mut stmt = conn.prepare(sql).unwrap();
    let total: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    total.unwrap()
}

pub fn get_floating_stat_internal(conn: &PooledConnection<SqliteConnectionManager>, sql: &str) -> f64 {
    let mut stmt = conn.prepare(sql).unwrap();
    let total: Result<f64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    total.unwrap()
}

// Don't judge me. I can feel you judging me..
// It was the only way I could figure out to make these damn calls happen in parallel!
// The whole API is due a good refactor, I'll figure it out later...
pub async fn set_stats_internal(pool:  web::Data<ConnectionPool>) {
    let conn1 = pool.connection.clone().get().unwrap();

    let blocks = task::spawn_blocking(move || {
        get_blocks_internal(&conn1)
    }).await.unwrap();

    let current_block = CURRENT_STATS.read().unwrap().total_blocks;

    if blocks > current_block {

        let conn2 = pool.connection.clone().get().unwrap();
        let conn3 = pool.connection.clone().get().unwrap();
        let conn4 = pool.connection.clone().get().unwrap();
        let conn5 = pool.connection.clone().get().unwrap();
        let conn6 = pool.connection.clone().get().unwrap();

        let transactions = task::spawn_blocking(move || {
            get_transactions_internal(&conn2)
        });

        let sysfees = task::spawn_blocking(move || {
            get_sysfee_internal(&conn3)
        });

        let transfers = task::spawn_blocking(move || {
            get_transfers_internal(&conn4)
        });

        let senders = task::spawn_blocking(move || {
            get_senders_internal(&conn5)
        });

        let contracts = task::spawn_blocking(move || {
            get_contracts_internal(&conn6)
        });

        let results = tokio::join!(transactions, sysfees, transfers, senders, contracts);

        {
            let mut w = CURRENT_STATS.write().unwrap();

            w.total_blocks = blocks;
            w.total_transactions = results.0.unwrap();
            w.total_sysfee = results.1.unwrap();
            w.total_transfers = results.2.unwrap();
            w.total_senders = results.3.unwrap();
            w.total_contracts = results.4.unwrap();
        }
    } else {
        // println!("No cache updated needed.")
    }
}

pub fn get_blocks_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let sql = "SELECT id FROM blocks WHERE id=(SELECT max(id) FROM blocks)";
    get_unsigned_stat_internal(conn, sql)
}

pub fn get_transactions_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let sql = "SELECT id FROM transactions WHERE id=(SELECT max(id) FROM transactions)";
    get_unsigned_stat_internal(conn, sql)
}

pub fn get_sysfee_internal(conn: &PooledConnection<SqliteConnectionManager>) -> f64 {
    let sql = "SELECT sum(sysfee) FROM transactions";
    get_floating_stat_internal(conn, sql) / PRECISION
}

pub fn get_transfers_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let sql = "SELECT SUM(LENGTH(notifications) - LENGTH(REPLACE(notifications, 'Transfer', ''))) / 8 FROM transactions WHERE notifications LIKE '%Transfer%'";
    get_unsigned_stat_internal(conn, sql)
}

pub fn get_senders_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let sql = "SELECT COUNT(DISTINCT sender) FROM transactions";
    get_unsigned_stat_internal(conn, sql)
}

pub fn get_contracts_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let deploy_event = r#"'%"contract":"0xfffdc93764dbaddd97c48f252a53ea4643faa3fd","eventname":"Deploy"%'"#;
    let sql = "SELECT COUNT() FROM transactions WHERE notifications LIKE ".to_string() + deploy_event;
    get_unsigned_stat_internal(conn, &sql) + 9 // fetch natives properly in future
}
