use super::JarRuntime;

#[derive(Debug, Clone)]
pub struct ContainerRuntime;

impl JarRuntime for ContainerRuntime {
    fn run(&self, world: &str) -> anyhow::Result<()> {
        todo!()
    }

    fn clean(&self, world: &str) -> anyhow::Result<()> {
        todo!()
    }
}

impl Default for ContainerRuntime {
    fn default() -> Self {
        todo!()
    }
}
