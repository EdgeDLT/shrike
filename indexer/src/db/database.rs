use rusqlite::{params, Connection, Result};
use log::info;

use crate::config::AppConfig;

use super::model::{Block, Transaction};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(config: &AppConfig) -> Result<Self> {

        if config.test_db {
            info!("Using test database.");
            let conn = Connection::open("shrike_test.db3")?;

            Ok(Database { conn })

        } else {
            info!("Using database at {}.", config.db_path);
            let conn = Connection::open(&config.db_path)?;

            Ok(Database { conn })
        }
    }

    pub fn set_to_wal(&self) -> Result<()> {
        let wal_active: String = self.conn.query_row("PRAGMA journal_mode", [], |row| row.get(0))?;
        if wal_active != "wal" {
            let _: String = self.conn.query_row("PRAGMA journal_mode=WAL", [], |row| row.get(0))?;
            info!("Set db to WAL mode.");
        } else {
            info!("WAL mode already active.");
        }

        Ok(())
    }

    pub fn create_index(&self, name: &str, table: &str, column: &str) -> Result<usize> {
        let sql = format!("CREATE INDEX IF NOT EXISTS {} ON {} ({})", name, table, column);
        let result = self.conn.execute(&sql, [])?;

        Ok(result)
    }

    pub fn create_block_table(&self) -> Result<usize> {
        let result = self.conn.execute("CREATE TABLE IF NOT EXISTS blocks (
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

    pub fn create_transaction_table(&self) -> Result<usize> {
        let result = self.conn.execute("CREATE TABLE IF NOT EXISTS transactions (
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

    pub fn insert_into_block_table(&self, block: Block) -> Result<usize> {
        let sql = "INSERT INTO blocks (
            id, hash, size, version, merkle_root, time,
            nonce, speaker, next_consensus, reward, reward_receiver, witnesses
        ) VALUES (0, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)";

        let result = self.conn.execute(sql, params![
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

    // synced rollback for both tables
    pub fn insert_blocks_transactions(&self, blocks: impl Iterator<Item = Block>, transactions: impl Iterator<Item = Transaction>) -> Result<()> {
        let tx = self.conn.unchecked_transaction()?;

        let mut block_stmt = self.conn.prepare_cached("INSERT INTO blocks (
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

        let mut tx_stmt = self.conn.prepare_cached( "INSERT INTO transactions (
            hash, block_index, vm_state, size, version, nonce, sender, sysfee, netfee,
            valid_until, signers, script, witnesses, stack_result, notifications
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)")?;

        for transaction in transactions {
            let _ = tx_stmt.execute(params![
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
            ]);
        }

        let result = tx.commit();
        if let Err(e) = result {
            println!("Error committing transaction: {:?}", e);
        }

        Ok(())
    }

    pub fn get_last_index(&self, table: &str) -> Result<u64> {
        let sql = &format!("SELECT id FROM {} WHERE id=(SELECT max(id) FROM {})", table, table);
        let mut stmt = self.conn.prepare(sql)?;
        let index: u64 = stmt.query_row([], |row| row.get(0))?;

        Ok(index)
    }

    #[allow(dead_code)]
    pub fn drop_table(&self, table: &str) -> Result<usize> {
        let result = self.conn.execute(&format!("DROP TABLE {}", table), [])?;

        Ok(result)
    }
}