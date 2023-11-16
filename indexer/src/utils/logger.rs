use anyhow::Result;
use log::LevelFilter;

use std::io::{self, Write};

use crate::config::AppConfig;

pub fn init() -> Result<()> {
    let config = AppConfig::new();
    let level = match config.log_level.as_str() {
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Off,
    };

    env_logger::builder()
        .filter_level(level)
        .format_timestamp(None)
        .init();

    Ok(())
}

pub fn inline_print(message: &str) {
    print!("{}", message);
    io::stdout().flush().unwrap();
}
