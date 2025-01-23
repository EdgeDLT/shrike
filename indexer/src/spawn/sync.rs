use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader, Lines};
use tokio::process::Command;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use log::warn;
use crate::utils::{logger, node};
use regex::Regex;
use std::path::Path;

pub async fn run_node(
    max_height: u64,
) -> Result<
    (
        Lines<BufReader<File>>,
        JoinHandle<()>,
        oneshot::Sender<()>,
    ),
    anyhow::Error,
> {
    let re = Regex::new(r#""headerHeight": (\d+),"#).unwrap();
    let log_path = Path::new("./log/neogo.log");

    // Start the node process
    let mut cmd = Command::new(node::NEOGO_PATH);
    let mut node = cmd
        .args(["node", "-m"])
        .spawn()
        .expect("Failed to run node");

    // Wait for log file to be created
    while !log_path.exists() {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    // Open and read the log file
    let file = File::open(log_path).await?;
    let mut reader = BufReader::new(file).lines();

    while let Some(line) = reader.next_line().await.unwrap_or_default() {
        if line.contains("headerHeight") {
            if let Some(caps) = re.captures(&line) {
                let height = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
                logger::inline_print(&format!("\rCurrent height: {height}"));

                if max_height != 0 && height >= max_height {
                    warn!("Exceeded target height.");
                    break;
                }
            }
        } else {
            // println!("{}", line); // for debugging
        }

        if line.contains("synchronized") {
            println!();
            break;
        }
    }

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let handle = {
        tokio::spawn(async move {
            let _ = shutdown_rx.await;
            warn!("Shutdown signal received.");
            let _ = node.kill().await;
            warn!("Node killed.");
        })
    };

    Ok((reader, handle, shutdown_tx))
}
