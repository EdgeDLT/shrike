use lib::db::DB_PATH;

#[derive(Debug)]
pub struct AppConfig {
    pub test_db: bool,
    pub db_path: String,
    pub node_path: String,
    pub log_level: String,
    pub batch_size: u64,
    pub keep_alive: bool,
    pub keep_alive_interval: u64,
    pub height_limit: u64,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            test_db: false,
            db_path: DB_PATH
                .to_str()
                .expect("Failed to convert path")
                .to_string(),
            node_path: String::from("http://localhost:10332"),
            log_level: String::from("info"),
            batch_size: 50,
            keep_alive: false,
            keep_alive_interval: 5,
            height_limit: 0,
        }
    }
}
