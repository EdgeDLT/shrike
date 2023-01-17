mod block;
mod transaction;
mod stat;
mod error;

use actix_web::{web, http::header, App, HttpServer};
use actix_cors::Cors;
use rusqlite::OpenFlags;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;

const DB_PATH: &str = "../shrike-indexer/shrike.db3";

struct ConnectionPool {
    connection: Pool<SqliteConnectionManager>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let manager = SqliteConnectionManager::file(DB_PATH)
        .with_flags(OpenFlags::SQLITE_OPEN_READ_ONLY);
    let pool = Pool::new(manager).unwrap();

    let connection_pool = web::Data::new(ConnectionPool {
        connection: pool
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(connection_pool.clone())
            .configure(block::controller::config)
            .configure(transaction::controller::config)
            .configure(stat::controller::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
