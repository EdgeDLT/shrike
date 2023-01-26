use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

const PRECISION: f64 = 100000000.0;

pub fn get_blocks_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let sql = "SELECT id FROM blocks WHERE id=(SELECT max(id) FROM blocks)";
    let mut stmt = conn.prepare(sql).unwrap();

    let total: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    total.unwrap()
}

pub fn get_transactions_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let sql = "SELECT id FROM transactions WHERE id=(SELECT max(id) FROM transactions)";
    let mut stmt = conn.prepare(sql).unwrap();

    let total: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    total.unwrap()
}

pub fn get_sysfee_internal(conn: &PooledConnection<SqliteConnectionManager>) -> f64 {
    let sql = "SELECT sum(sysfee) FROM transactions";
    let mut stmt = conn.prepare(sql).unwrap();

    let res: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));
    let total = res.unwrap() as f64 / PRECISION;

    total
}

pub fn get_transfers_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let sql = "SELECT SUM(LENGTH(notifications) - LENGTH(REPLACE(notifications, 'Transfer', ''))) / 8 FROM transactions WHERE notifications LIKE '%Transfer%'";
    let mut stmt = conn.prepare(sql).unwrap();

    let total: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    total.unwrap()
}

pub fn get_senders_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let sql = "SELECT COUNT(DISTINCT sender) FROM transactions";
    let mut stmt = conn.prepare(&sql).unwrap();

    let senders: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    senders.unwrap()
}

pub fn get_contracts_internal(conn: &PooledConnection<SqliteConnectionManager>) -> u64 {
    let deploy_event = r#"'%"contract":"0xfffdc93764dbaddd97c48f252a53ea4643faa3fd","eventname":"Deploy"%'"#;

    let sql = "SELECT COUNT() FROM transactions WHERE notifications LIKE ".to_string() + deploy_event;
    let mut stmt = conn.prepare(&sql).unwrap();

    let contracts: Result<u64, rusqlite::Error> = stmt.query_row([], |row| row.get(0));

    contracts.unwrap() + 9 // fetch natives properly in future
}
