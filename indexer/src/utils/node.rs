use log::{info, warn};
use text_io::read;
use tokio::{fs::File, io::AsyncWriteExt};

use std::{env, io, path::Path, process::Command};

// move these to config in future
#[cfg(target_os = "linux")]
pub static NEOGO_PATH: &str = "./neogo";
#[cfg(target_os = "macos")]
pub static NEOGO_PATH: &str = "./neogo";
#[cfg(target_os = "windows")]
pub static NEOGO_PATH: &str = "./neogo.exe";

#[cfg(target_os = "linux")]
static NEOGO_DL: &str =
    "https://github.com/nspcc-dev/neo-go/releases/download/v0.104.0/neo-go-linux-amd64";
#[cfg(target_os = "macos")]
static NEOGO_DL: &str =
    "https://github.com/nspcc-dev/neo-go/releases/download/v0.104.0/neo-go-darwin-arm64";
#[cfg(target_os = "windows")]
static NEOGO_DL: &str =
    "https://github.com/nspcc-dev/neo-go/releases/download/v0.104.0/neo-go-windows-amd64.exe";

pub async fn check_neogo() -> io::Result<()> {
    let path = Path::new(NEOGO_PATH);
    if !path.exists() {
        warn!("NeoGo not found in directory. Install? (y/n)");
        let answer: char = read!();
        assert!((answer == 'y'), "User declined to install NeoGo.");

        let mut file = File::create(path).await?;
        let mut response = reqwest::get(NEOGO_DL).await.unwrap();
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
        info!("NeoGo installed.");
    }
    Ok(())
}
