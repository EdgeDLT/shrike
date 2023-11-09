use tokio::process::{Command, ChildStderr};
use tokio::io::{BufReader, AsyncBufReadExt, Lines};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use log::{warn, info};

use std::process::Stdio;

pub async fn sync_node(path: &str) -> Result<(Lines<BufReader<ChildStderr>>, JoinHandle<()>, oneshot::Sender<()>), anyhow::Error> {
    let mut cmd = Command::new(path);
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
            info!("{}", line);
        }
        if line.contains("synchronized") {
            break;
        }
    };

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