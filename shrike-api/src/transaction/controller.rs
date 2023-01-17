use actix_web::{get, web, Responder, HttpResponse};
use crate::ConnectionPool;
use crate::error::Error;
use super::models::Transaction;

#[get("/v1/transaction/{hash}")]
async fn get_transaction(pool: web::Data<ConnectionPool>, path: web::Path<String>) -> impl Responder {
    let con = &pool.connection.get().unwrap();
    let hash = path.into_inner();

    let sql = "SELECT * FROM transactions WHERE hash = ?";
    let mut stmt = con.prepare(sql).unwrap();

    let result = stmt.query_row([hash], |row| {
        Ok(Transaction {
            index: row.get(0)?,
            hash: row.get(1)?,
            block_hash: row.get(2)?,
            vm_state: row.get(3)?,
            size: row.get(4)?,
            version: row.get(5)?,
            nonce: row.get(6)?,
            sender: row.get(7)?,
            sysfee: row.get(8)?,
            netfee: row.get(9)?,
            valid_until: row.get(10)?,
            signers: row.get(11)?,
            script:row.get(12)?,
            witnesses: row.get(13)?,
            stack_result: row.get(14)?,
            notifications: row.get(15)?
        })
    });

    match result {
        Ok(transaction) => HttpResponse::Ok().json(transaction),
        Err(_) => HttpResponse::Ok().json(Error { error: "Transaction does not exist.".to_string() })
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(get_transaction);
}
