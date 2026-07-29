use anyhow::anyhow;

pub trait JarRuntime {
    fn run(&self, world: &str) -> anyhow::Result<()>;
    fn clean(&self, world: &str) -> anyhow::Result<()>;
}
