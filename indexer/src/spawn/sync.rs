use tokio::process::{Command, ChildStderr};
use tokio::io::{BufReader, AsyncBufReadExt, Lines};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use log::{warn, info};

use std::process::Stdio;
use regex::Regex;
use crate::utils::node;

pub async fn sync_node(max_height: u64) -> Result<(Lines<BufReader<ChildStderr>>, JoinHandle<()>, oneshot::Sender<()>), anyhow::Error> {
    let re = Regex::new(r#""headerHeight": (\d+),"#).unwrap();
    let mut cmd = Command::new(node::NEOGO_PATH);
    cmd
        .stderr(Stdio::piped());

    let mut node = cmd
        .args(["node", "-m"])
        .spawn()
        .expect("Failed to run node");

    let stderr = node.stderr.take().expect("No stderr for node");
    let mut stderr_reader = BufReader::new(stderr).lines();

    while let Some(line) = stderr_reader.next_line().await.unwrap_or_default() {
        if line.contains("headerHeight") {

            if let Some(caps) = re.captures(&line) {
                let height = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
                info!("Current node height: {}", height);
                if max_height != 0 && height >= max_height {
                    warn!("Exceeded target height.");
                    break;
                } else {
                    continue;
                }
            }
        }

        if line.contains("synchronized") {
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

    Ok((stderr_reader, handle, shutdown_tx))
}