pub mod container_runtime;
pub mod nixbox_runtime;
pub mod prelude;

use prelude::*;

use std::str::FromStr;

use container_runtime::ContainerRuntime;
#[cfg(target_os = "linux")]
use nixbox_runtime::NixboxRuntime;

use anyhow::anyhow;
// ==========================================================================================

#[derive(Clone)]
pub enum Runtimes {
    Container(ContainerRuntime),
    #[cfg(target_os = "linux")]
    Nixbox(NixboxRuntime),
}

impl JarRuntime for Runtimes {
    fn run(&self, world: &str) -> anyhow::Result<()> {
        match self {
            Runtimes::Container(runtime) => runtime.run(world),
            #[cfg(target_os = "linux")]
            Runtimes::Nixbox(runtime) => runtime.run(world),
        }
    }

    fn clean(&self, world: &str) -> anyhow::Result<()> {
        match self {
            Runtimes::Container(runtime) => runtime.clean(world),
            #[cfg(target_os = "linux")]
            Runtimes::Nixbox(runtime) => runtime.clean(world),
        }
    }
}

impl FromStr for Runtimes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "container" => Ok(Runtimes::Container(ContainerRuntime)),
            #[cfg(target_os = "linux")]
            "nixbox" => Ok(Runtimes::Nixbox(NixboxRuntime::default())),
            _ => Err(anyhow!("Unknown runtime (or not supported): {s}")),
        }
    }
}
