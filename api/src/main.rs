mod block;
mod error;
mod shared;
mod stat;
mod transaction;

use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};
use lib::db::DB_PATH;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OpenFlags;
use tokio::{task, time};

use std::time::Duration;

const REFRESH_INTERVAL: u64 = 3; // how often we check for a new block and refresh stats in seconds

pub struct ConnectionPool {
    connection: Pool<SqliteConnectionManager>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_path = DB_PATH
        .to_str()
        .expect("Failed to convert database path to str");
    let manager =
        SqliteConnectionManager::file(db_path).with_flags(OpenFlags::SQLITE_OPEN_READ_ONLY);
    let pool = Pool::new(manager).unwrap();

    let connection_pool = web::Data::new(ConnectionPool { connection: pool });

    let internal_connection = connection_pool.clone();

    task::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(REFRESH_INTERVAL));
        loop {
            let c = internal_connection.clone();
            interval.tick().await;
            stat::internals::set_stats_internal(c).await;
        }
    });

    println!("Opening to requests on http://0.0.0.0:8080.");

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
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
