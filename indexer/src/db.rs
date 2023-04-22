use log::info;
use rusqlite::{Connection, params};

use lib::db::DB_PATH;
use crate::models;

pub fn connect_to_db() -> Connection {
    let db_path = DB_PATH.to_str().expect("Failed to convert database path to str");
    Connection::open(db_path).unwrap()
}

pub fn set_to_wal() -> Result<(), anyhow::Error> {
    let conn = connect_to_db();
    let wal_active: String = conn.query_row("PRAGMA journal_mode", [], |row| row.get(0))?;
    if wal_active != "wal" {
        let _: String = conn.query_row("PRAGMA journal_mode=WAL", [], |row| row.get(0))?;
        info!("Set db to WAL mode.");
    }

    Ok(())
}

pub fn create_index(name: &str, table: &str, column: &str) -> Result<usize, anyhow::Error> {
    let conn = connect_to_db();
    let sql = format!("CREATE INDEX IF NOT EXISTS {} ON {} ({})", name, table, column);

    let result = conn.execute(&sql, [])?;

    Ok(result)
}

pub fn create_block_table() -> Result<usize, anyhow::Error> {
    let conn = connect_to_db();

    let result = conn.execute("CREATE TABLE IF NOT EXISTS blocks (
        id                  INTEGER PRIMARY KEY AUTOINCREMENT,
        hash                TEXT NOT NULL UNIQUE,
        size                INTEGER NOT NULL,
        version             INTEGER NOT NULL,
        merkle_root         TEXT NOT NULL,
        time                INTEGER NOT NULL,
        nonce               TEXT NOT NULL,
        speaker             INTEGER NOT NULL,
        next_consensus      TEXT NOT NULL,
        reward              FLOAT NOT NULL,
        reward_receiver     TEXT NOT NULL,
        witnesses           TEXT NOT NULL
        )", [])?;

    Ok(result)
}

pub fn insert_into_block_table(block: models::Block) -> Result<usize, anyhow::Error> {
    let conn = connect_to_db();
    let sql = "INSERT INTO blocks (
        id, hash, size, version, merkle_root, time,
        nonce, speaker, next_consensus, reward, reward_receiver, witnesses
    ) VALUES (0, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)";

    let result = conn.execute(sql, params![
        block.hash,
        block.size,
        block.version,
        block.merkle_root,
        block.time,
        block.nonce,
        block.speaker,
        block.next_consensus,
        block.reward,
        block.reward_receiver,
        block.witnesses
    ])?;

    Ok(result)
}

pub fn create_transaction_table() -> Result<usize, anyhow::Error> {
    let conn = connect_to_db();

    let result = conn.execute("CREATE TABLE IF NOT EXISTS transactions (
        id                  INTEGER PRIMARY KEY AUTOINCREMENT,
        hash                TEXT NOT NULL UNIQUE,
        block_index         INTEGER NOT NULL,
        vm_state            TEXT NOT NULL,
        size                INTEGER NOT NULL,
        version             INTEGER NOT NULL,
        nonce               INTEGER NOT NULL,
        sender              TEXT NOT NULL,
        sysfee              TEXT NOT NULL,
        netfee              TEXT NOT NULL,
        valid_until         INTEGER NOT NULL,
        signers             TEXT NOT NULL,
        script              TEXT NOT NULL,
        witnesses           TEXT NOT NULL,
        stack_result        TEXT,
        notifications       TEXT,
        FOREIGN KEY (block_index) REFERENCES blocks (id)
        )", [])?;

    Ok(result)
}

// ugly but gives an extra speed up
pub fn insert_blocks_transactions(blocks: impl Iterator<Item = models::Block>, transactions: impl Iterator<Item = models::Transaction>) -> Result<(), anyhow::Error> {
    let conn = connect_to_db();
    let tx = conn.unchecked_transaction()?;

    let mut block_stmt = conn.prepare_cached("INSERT INTO blocks (
        hash, size, version, merkle_root, time,
        nonce, speaker, next_consensus, reward, reward_receiver, witnesses
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)")?;

    for block in blocks {
        block_stmt.execute(params![
            block.hash,
            block.size,
            block.version,
            block.merkle_root,
            block.time,
            block.nonce,
            block.speaker,
            block.next_consensus,
            block.reward,
            block.reward_receiver,
            block.witnesses
        ])?;
    }

    let mut tx_stmt = conn.prepare_cached( "INSERT INTO transactions (
        hash, block_index, vm_state, size, version, nonce, sender, sysfee, netfee,
        valid_until, signers, script, witnesses, stack_result, notifications
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)")?;

    for transaction in transactions {
        tx_stmt.execute(params![
            transaction.hash,
            transaction.block_index,
            transaction.vm_state,
            transaction.size,
            transaction.version,
            transaction.nonce,
            transaction.sender,
            transaction.sysfee,
            transaction.netfee,
            transaction.valid_until,
            transaction.signers,
            transaction.script,
            transaction.witnesses,
            transaction.stack_result,
            transaction.notifications
        ])?;
    }
    tx.commit()?;

    Ok(())
}

pub fn get_last_index(table: &str) -> Result<u64, anyhow::Error> {
    let conn = connect_to_db();
    let sql = &format!("SELECT id FROM {} WHERE id=(SELECT max(id) FROM {})", table, table);

    let mut stmt = conn.prepare(sql).unwrap();
    let index: u64 = stmt.query_row([], |row| row.get(0))?;

    Ok(index)
}

#[allow(dead_code)]
pub fn drop_table(table: &str) -> Result<usize, anyhow::Error> {
    let conn = connect_to_db();
    let result = conn.execute(&format!("DROP TABLE {}", table), [])?;

    Ok(result)
}
