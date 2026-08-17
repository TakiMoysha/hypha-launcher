use std::path::PathBuf;

use async_trait::async_trait;

#[derive(Debug)]
pub struct JarArguments {
    pub universe: String,
    pub jvm_opts: Vec<String>,
    pub server_jar: PathBuf,
    pub server_opts: Vec<String>,
    pub work_dir: PathBuf,
}

impl JarArguments {
    pub fn new(
        universe: &str,
        jvm_opts: Option<Vec<String>>,
        server_jar: PathBuf,
        server_opts: Option<Vec<String>>,
        work_dir: PathBuf,
    ) -> Self {
        Self {
            universe: universe.to_string(),
            jvm_opts: jvm_opts.unwrap_or_default(),
            server_jar,
            server_opts: server_opts.unwrap_or_default(),
            work_dir,
        }
    }
}

#[async_trait]
pub trait JarRuntime {
    async fn run(&self, args: JarArguments) -> anyhow::Result<()>;
    async fn clean(&self) -> anyhow::Result<()>;
}
