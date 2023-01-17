use tokio::process::{Command, Child};
use tokio::io::{BufReader, AsyncBufReadExt};

use std::process::{Command as CommandSync, Stdio};

pub struct NeoGo {
    binary_path: &'static str,
}

impl NeoGo {
    pub fn new(binary_path: &'static str) -> Self {
        NeoGo { binary_path }
    }

    #[allow(dead_code)]
    pub fn convert(&self, arg: &str) -> String {
        let args = ["util", "convert", arg];
        let output = CommandSync::new(self.binary_path)
            .args(&args)
            .output()
            .expect("Failed to run binary");

        String::from_utf8_lossy(&output.stdout).into_owned()
    }

    pub async fn sync_node(&self) -> Result<Child, Box<dyn std::error::Error>> {
        let mut cmd = Command::new(self.binary_path);
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

}
