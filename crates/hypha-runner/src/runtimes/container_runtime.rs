use super::JarRuntime;

#[derive(Clone)]
pub struct ContainerRuntime;

impl JarRuntime for ContainerRuntime {
    fn run(&self, world: &str) -> anyhow::Result<()> {
        todo!()
    }

    fn clean(&self, world: &str) -> anyhow::Result<()> {
        todo!()
    }
}
