use log::{error, info, warn};
use text_io::read;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::config::AppConfig;
use std::{env, path::Path, process::Command};

// move these to config in future
#[cfg(target_os = "linux")]
pub static NEOGO_PATH: &str = "./neogo";
#[cfg(target_os = "macos")]
pub static NEOGO_PATH: &str = "./neogo";
#[cfg(target_os = "windows")]
pub static NEOGO_PATH: &str = "./neogo.exe";

fn get_neogo_release_notes(config: &AppConfig) -> String {
    #[cfg(target_os = "linux")]
    {
        format!(
            "https://github.com/nspcc-dev/neo-go/releases/tag/{}",
            config.node_version
        )
    }
    #[cfg(target_os = "macos")]
    {
        format!(
            "https://github.com/nspcc-dev/neo-go/releases/tag/{}",
            config.node_version
        )
    }
    #[cfg(target_os = "windows")]
    {
        format!(
            "https://github.com/nspcc-dev/neo-go/releases/tag/{}",
            config.node_version
        )
    }
}

fn get_neogo_dl(config: &AppConfig) -> String {
    #[cfg(target_os = "linux")]
    {
        format!(
            "https://github.com/nspcc-dev/neo-go/releases/download/{}/neo-go-linux-amd64",
            config.node_version
        )
    }
    #[cfg(target_os = "macos")]
    {
        format!(
            "https://github.com/nspcc-dev/neo-go/releases/download/{}/neo-go-darwin-arm64",
            config.node_version
        )
    }
    #[cfg(target_os = "windows")]
    {
        format!(
            "https://github.com/nspcc-dev/neo-go/releases/download/{}/neo-go-windows-amd64.exe",
            config.node_version
        )
    }
}

pub async fn check_neogo(config: &AppConfig) -> Result<(), anyhow::Error> {
    let path = Path::new(NEOGO_PATH);
    if !path.exists() {
        warn!("NeoGo not found in directory. Install? (y/n)");
        let answer: char = read!();
        assert!((answer == 'y'), "User declined to install NeoGo.");

        let mut file = File::create(path).await?;
        let mut response = reqwest::get(&get_neogo_dl(config)).await.unwrap();
        while let Some(chunk) = response.chunk().await.unwrap() {
            file.write_all(&chunk).await?;
        }

        if env::consts::OS != "windows" {
            info!("Updating permissions..");
            Command::new("chmod")
                .arg("+x")
                .arg(NEOGO_PATH)
                .output()
                .expect("failed to update permissions");
        }
    } else {
        info!("NeoGo already installed.");
        let installed_version = check_neogo_version()?;
        let expected_version = config.node_version.to_string();

        if installed_version != expected_version {
            error!("Incorrect NeoGo version detected. Remove {} and re-run to install the correct version.", NEOGO_PATH);
            error!("Check the NeoGo version release notes at {} to see if chain state data is compatible.", get_neogo_release_notes(config));
            return Err(anyhow::anyhow!(
                "NeoGo version mismatch. Expected {}, got {}.",
                expected_version,
                installed_version
            ));
        }
    }
    Ok(())
}

pub fn check_neogo_version() -> Result<String, anyhow::Error> {
    let command_output = Command::new(NEOGO_PATH)
        .arg("-v")
        .output()
        .expect("Failed to execute version check");

    if !command_output.status.success() {
        return Err(anyhow::anyhow!("NeoGo version check failed."));
    }

    let mut lines = command_output.stdout.split(|b| *b == b'\n');
    let version_line = lines.find(|line| line.starts_with(b"Version:"));

    if let Some(version_line) = version_line {
        let version = String::from_utf8_lossy(version_line);
        let version = format!("v{}", version.trim().split(' ').nth(1).unwrap());
        Ok(version)
    } else {
        Err(anyhow::anyhow!("NeoGo version check failed."))
    }
}
