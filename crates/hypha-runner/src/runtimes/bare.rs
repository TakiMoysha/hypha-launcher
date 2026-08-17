use super::{JarArguments, JarRuntime};
use async_trait::async_trait;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tracing::{debug, info};

#[derive(Debug, Clone)]
pub struct BareRuntime;

#[async_trait]
impl JarRuntime for BareRuntime {
    #[tracing::instrument(name = "[bare:run]")]
    async fn run(&self, args: JarArguments) -> anyhow::Result<()> {
        info!("args.work_dir: {:#?}", args);

        let mut cmd = Command::new("java");

        cmd.args(&args.jvm_opts)
            .arg("-jar")
            .arg(&args.server_jar)
            .arg("--universe")
            .arg(&args.universe)
            .args(&args.server_opts)
            .current_dir(&args.work_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        info!("cmd: {:#?}", cmd);

        let mut child = cmd.spawn()?;

        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        if let Some(stdout) = stdout {
            let reader = BufReader::new(stdout);
            tokio::spawn(async move {
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    tracing::info!(target: "HYTALE_SERVER", "{}", line);
                }
            });
        }

        if let Some(stderr) = stderr {
            let reader = BufReader::new(stderr);
            tokio::spawn(async move {
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    tracing::error!(target: "HYTALE_SERVER", "{}", line);
                }
            });
        }

        let status = child.wait().await?;

        if cfg!(debug_assertions) {
            println!("Hytale server exited with status: {}", status);
        }

        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Hytale server exited with status: {}",
                status
            ))
        }
    }

    async fn clean(&self) -> anyhow::Result<()> {
        todo!()
    }
}

impl Default for BareRuntime {
    fn default() -> Self {
        Self
    }
}
