use tokio::process::{Command, Child};
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::sync::{oneshot, Mutex};
use tokio::task::JoinHandle;

use std::process::Stdio;
use std::sync::Arc;

pub async fn sync_node(
    path: &str,
) -> Result<(Arc<Mutex<Child>>, JoinHandle<()>, oneshot::Sender<()>), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(path);
    cmd.stderr(Stdio::piped());

    let mut node = cmd
        .kill_on_drop(true)
        .args(["node", "-m"])
        .spawn()
        .expect("Failed to run node");

    let stderr = node.stderr.take().expect("No stderr for node");

    let mut stderr_reader = BufReader::new(stderr).lines();

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

    let node_arc = Arc::new(Mutex::new(node));

    let stderr_handle = {
        let node_shared = Arc::clone(&node_arc);
        tokio::spawn(async move {
            while let Some(line) = stderr_reader.next_line().await.unwrap_or_default() {
                if line.contains("headerHeight") {
                    println!("{}", line);
                }
                if line.contains("synchronized") {
                    break;
                }
            }
            // Wait for the shutdown signal
            let _ = shutdown_rx.await;
            // Terminate the child process
            let mut node_locked = node_shared.lock().await;
            let _ = node_locked.kill().await;
        })
    };

    Ok((node_arc, stderr_handle, shutdown_tx))
}
