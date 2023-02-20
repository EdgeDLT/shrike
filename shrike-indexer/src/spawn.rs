use tokio::process::{Command, Child};
use tokio::io::{BufReader, AsyncBufReadExt};

use std::process::Stdio;

pub async fn sync_node(path: &str) -> Result<Child, Box<dyn std::error::Error>> {
    let mut cmd = Command::new(path);
    cmd.stderr(Stdio::piped());

    let mut node = cmd
        .kill_on_drop(true)
        .args(["node", "-m"])
        .spawn()
        .expect("Failed to run node");

    let stderr = node.stderr
        .take()
        .expect("No stderr for node");

    let mut stderr_reader = BufReader::new(stderr).lines();

    while let Some(line) = stderr_reader.next_line().await? {
        if line.contains("headerHeight") {
            println!("{}", line)
        }
        if line.contains("synchronized") {
            break;
        }
    }
    Ok(node)
}
