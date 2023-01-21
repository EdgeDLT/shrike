use actix_web::{get, web, Responder, HttpResponse};
use crate::ConnectionPool;
use crate::error::Error;
use super::models::Block;

#[get("/v1/block/{id}")]
async fn get_block(pool: web::Data<ConnectionPool>, path: web::Path<String>) -> impl Responder {
    let con = &pool.connection.get().unwrap();
    let hash = path.into_inner();

    match hash.trim().parse::<u64>() {
        Ok(id) => {
            let sql = "SELECT * FROM blocks WHERE id = ?";
            let mut stmt = con.prepare(sql).unwrap();

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

            match result {
                Ok(block) => HttpResponse::Ok().json(block),
                Err(_) => HttpResponse::Ok().json(Error { error: "Block does not exist.".to_string() })
            }
        },
        Err(_) => {
            let sql = "SELECT * FROM blocks WHERE hash = ?";
            let mut stmt = con.prepare(sql).unwrap();

            let result = stmt.query_row([hash], |row| {
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

            match result {
                Ok(block) => HttpResponse::Ok().json(block),
                Err(_) => HttpResponse::Ok().json(Error { error: "Block does not exist.".to_string() })
            }
        }
    }


}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_block);
}

