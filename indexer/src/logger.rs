use log::LevelFilter;
use anyhow::Result;

pub fn init(level: LevelFilter) -> Result<()> {
    env_logger::builder()
        .filter_level(level)
        .format_timestamp(None)
        .init();

    Ok(())
}