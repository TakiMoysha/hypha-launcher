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
// ==========================================================================================

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
}

impl JarRuntime for Runtimes {
    fn run(&self, world: &str) -> anyhow::Result<()> {
        self.inner().run(world)
    }
    fn clean(&self, world: &str) -> anyhow::Result<()> {
        self.inner().clean(world)
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
