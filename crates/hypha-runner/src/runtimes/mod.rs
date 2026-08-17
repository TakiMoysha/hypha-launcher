pub mod bare;
pub mod container;
#[cfg(target_os = "linux")]
pub mod nixbox;
pub mod prelude;

use prelude::*;

use std::str::FromStr;

use bare::BareRuntime;
use container::ContainerRuntime;
#[cfg(target_os = "linux")]
use nixbox::NixboxRuntime;

use anyhow::anyhow;

use crate::config::RunnerConfig;

#[derive(Debug, Clone)]
pub enum Runtimes {
    Bare(BareRuntime),
    Container(ContainerRuntime),
    #[cfg(target_os = "linux")]
    Nixbox(NixboxRuntime),
}

impl Runtimes {
    /// dispatch to inner runtime
    pub fn inner(&self) -> &dyn JarRuntime {
        match self {
            Runtimes::Bare(runtime) => runtime,
            Runtimes::Container(runtime) => runtime,
            #[cfg(target_os = "linux")]
            Runtimes::Nixbox(runtime) => runtime,
        }
    }

    #[tracing::instrument(name = "[runtimes:run]")]
    pub async fn run(&self, universe: &str, config: &RunnerConfig) -> anyhow::Result<()> {
        let latest_server_jar = config
            .server_jar_path("latest")
            .ok_or_else(|| anyhow!("Server jar path not found"))?;

        let jvm_opts = config.default_jvm_opts.clone();
        let server_opts = vec![];
        let args = JarArguments::new(
            universe,
            Some(jvm_opts),
            latest_server_jar,
            Some(server_opts),
            config.work_dir.clone(),
        );

        let _ = self.inner().run(args).await?;

        Ok(())
    }

    pub async fn clean(&self, universe: &str, config: &RunnerConfig) -> anyhow::Result<()> {
        self.inner().clean().await
    }
}

impl FromStr for Runtimes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "bare" => Ok(Runtimes::Bare(BareRuntime::default())),
            "container" => Ok(Runtimes::Container(ContainerRuntime::default())),
            #[cfg(target_os = "linux")]
            "nixbox" => Ok(Runtimes::Nixbox(NixboxRuntime::default())),
            #[cfg(not(target_os = "linux"))]
            "nixbox" => Err(anyhow!("Nixbox runtime is not supported on this platform")),
            _ => Err(anyhow!("Unknown runtime (or not supported): {s}")),
        }
    }
}
