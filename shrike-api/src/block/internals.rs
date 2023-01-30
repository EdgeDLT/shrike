use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;

use crate::error::Error;
use crate::shared::checker;
use super::models::Block;

pub fn get_block_internal(conn: &PooledConnection<SqliteConnectionManager>, path: String) -> Result<Block, Error> {

    match path.trim().parse::<u64>() {
        Ok(id) => {
            let sql = "SELECT * FROM blocks WHERE id = ?";
            let mut stmt = conn.prepare(sql).unwrap();

            let result = stmt.query_row([id], |row| {
                Ok(Block {
                    index: row.get(0)?,
                    hash: row.get(1)?,
                    size: row.get(2)?,
                    version: row.get(3)?,
                    merkle_root: row.get(4)?,
                    time: row.get(5)?,
                    nonce: row.get(6)?,
                    speaker: row.get(7)?,
                    next_consensus: row.get(8)?,
                    reward: row.get(9)?,
                    reward_receiver: row.get(10)?,
                    witnesses: row.get(11)?
                })
            });

            result.map_err(|_| Error { error: "Block does not exist.".to_string() })
        },
        Err(_) => {

            if !checker::is_neo_txid_hash(&path) {
                return Err(Error { error: "Invalid block hash.".to_string() });
            }

            let sql = "SELECT * FROM blocks WHERE hash = ?";
            let mut stmt = conn.prepare(sql).unwrap();

            let result = stmt.query_row([path], |row| {
                Ok(Block {
                    index: row.get(0)?,
                    hash: row.get(1)?,
                    size: row.get(2)?,
                    version: row.get(3)?,
                    merkle_root: row.get(4)?,
                    time: row.get(5)?,
                    nonce: row.get(6)?,
                    speaker: row.get(7)?,
                    next_consensus: row.get(8)?,
                    reward: row.get(9)?,
                    reward_receiver: row.get(10)?,
                    witnesses: row.get(11)?
                })
            });

            result.map_err(|_| Error { error: "Block does not exist.".to_string() })
        }
    }
}

pub fn get_block_time(conn: &PooledConnection<SqliteConnectionManager>, path: String) -> Result<u64, Error> {

    match path.trim().parse::<u64>() {
        Ok(id) => {
            let sql = "SELECT time FROM blocks WHERE id = ?";
            let mut stmt = conn.prepare(sql).unwrap();

            let result = stmt.query_row([id], |row| {
                Ok(row.get(0)?)
            });

            result.map_err(|_| Error { error: "Block does not exist.".to_string() })
        },
        Err(_) => {

            if !checker::is_neo_txid_hash(&path) {
                return Err(Error { error: "Invalid block hash.".to_string() });
            }

            let sql = "SELECT time FROM blocks WHERE hash = ?";
            let mut stmt = conn.prepare(sql).unwrap();

            let result = stmt.query_row([path], |row| {
                Ok(row.get(0)?)
            });

            result.map_err(|_| Error { error: "Block does not exist.".to_string() })
        }
    }
}
