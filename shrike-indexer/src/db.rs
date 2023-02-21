use rusqlite::{Connection, Error, params};

use crate::models;

const DB_PATH: &str = "shrike.db3";

pub fn connect_to_db() -> Connection {
    Connection::open(DB_PATH).unwrap()
}

pub fn create_block_table() -> usize {
    let conn = connect_to_db();

    conn.execute("CREATE TABLE IF NOT EXISTS blocks (
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
        )", []).unwrap()
}

#[allow(dead_code)]
pub fn drop_block_table() -> usize {
    let conn = connect_to_db();

    conn.execute("DROP TABLE blocks", []).unwrap()
}

pub fn get_last_block_index() -> Result<u64, Error> {
    let conn = connect_to_db();
    let sql = "SELECT id FROM blocks WHERE id=(SELECT max(id) FROM blocks)";

    let mut stmt = conn.prepare(sql).unwrap();
    let index: Result<u64, Error> = stmt.query_row([], |row| row.get(0));

    Ok(index?)
}

pub fn insert_into_block_table(block: models::Block) {
    let conn = connect_to_db();
    let sql = "INSERT INTO blocks (
        id, hash, size, version, merkle_root, time,
        nonce, speaker, next_consensus, reward, reward_receiver, witnesses
    ) VALUES (0, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)";

    conn.execute(sql, params![
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
    ]).unwrap();
}

pub fn create_transaction_table() -> usize {
    let conn = connect_to_db();

    conn.execute("CREATE TABLE IF NOT EXISTS transactions (
        id                  INTEGER PRIMARY KEY AUTOINCREMENT,
        hash                TEXT NOT NULL UNIQUE,
        block_hash          TEXT NOT NULL,
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
        notifications       TEXT
        )", []).unwrap()
}

#[allow(dead_code)]
pub fn drop_transaction_table() -> usize {
    let conn = connect_to_db();

    conn.execute("DROP TABLE transactions", []).unwrap()
}

#[allow(dead_code)]
pub fn get_last_transaction_index() -> Result<u64, Error> {
    let conn = connect_to_db();
    let sql = "SELECT id FROM transactions WHERE id=(SELECT max(id) FROM transactions)";

    let mut stmt = conn.prepare(sql).unwrap();
    let index: Result<u64, Error> = stmt.query_row([], |row| row.get(0));

    Ok(index?)
}

#[allow(dead_code)]
pub fn insert_into_transaction_table(transaction: models::Transaction) {
    let conn = connect_to_db();
    let sql = "INSERT INTO transactions (
        hash, block_hash, vm_state, size, version, nonce, sender, sysfee, netfee,
        valid_until, signers, script, witnesses, stack_result, notifications
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)";

    conn.execute(sql, params![
        transaction.hash,
        transaction.block_hash,
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
    ]).unwrap();
}

// ugly but gives an extra speed up
pub fn insert_blocks_transactions(blocks: impl Iterator<Item = models::Block>, transactions: impl Iterator<Item = models::Transaction>) -> Result<(), Error> {
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
        hash, block_hash, vm_state, size, version, nonce, sender, sysfee, netfee,
        valid_until, signers, script, witnesses, stack_result, notifications
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)")?;

    for transaction in transactions {
        tx_stmt.execute(params![
            transaction.hash,
            transaction.block_hash,
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
    tx.commit()
}
