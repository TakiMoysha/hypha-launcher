use super::{JarArguments, JarRuntime};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct ContainerRuntime;

#[async_trait]
impl JarRuntime for ContainerRuntime {
    async fn run(&self, args: JarArguments) -> anyhow::Result<()> {
        todo!()
    }

    async fn clean(&self) -> anyhow::Result<()> {
        todo!()
    }
}

impl Default for ContainerRuntime {
    fn default() -> Self {
        todo!()
    }
}
