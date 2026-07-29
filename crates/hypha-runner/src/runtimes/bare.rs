use super::JarRuntime;

#[derive(Debug, Clone)]
pub struct BareRuntime;

impl JarRuntime for BareRuntime {
    fn run(&self, world: &str) -> anyhow::Result<()> {
        todo!()
    }

    fn clean(&self, world: &str) -> anyhow::Result<()> {
        todo!()
    }
}

impl Default for BareRuntime {
    fn default() -> Self {
        todo!()
    }
}
