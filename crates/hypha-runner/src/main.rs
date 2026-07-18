use clap::{Command, arg};

fn main() {
    let args = Command::new("hypha-runner")
        .version("0.1.0")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("run")
                .about("Run a world")
                .arg_required_else_help(true)
                .arg(arg!([WORLD] "World name to run"))
                .arg(
                    arg!(--runtime <DRIVER> "Runtime to use (container, nixbox)")
                        .default_value("nixbox"),
                ),
        )
        .subcommand(Command::new("list").about("List existing worlds (WIP)"))
        .get_matches();

    println!("Hello, world! {:?}", args);
}

mod runtimes {
    use std::str::FromStr;

    use container_runtime::ContainerRuntime;
    #[cfg(target_os = "linux")]
    use nixbox_runtime::NixboxRuntime;

    use anyhow::anyhow;

    pub trait JarRuntime {
        fn run(&self, world: &str) -> anyhow::Result<()>;
    }

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
    }

    impl FromStr for Runtimes {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.trim().to_lowercase().as_str() {
                "container" => Ok(Runtimes::Container(ContainerRuntime)),
                #[cfg(target_os = "linux")]
                "nixbox" => Ok(Runtimes::Nixbox(NixboxRuntime)),
                #[cfg(not(target_os = "linux"))]
                "nixbox" => Err("Nixbox runtime is not available on this platform".into()),
                _ => Err(anyhow!("Unknown runtime: {s}")),
            }
        }
    }

    mod container_runtime {
        use super::JarRuntime;

        pub struct ContainerRuntime;

        impl JarRuntime for ContainerRuntime {
            fn run(&self, world: &str) -> anyhow::Result<()> {
                todo!()
            }
        }
    }

    mod nixbox_runtime {
        use super::JarRuntime;
        pub struct NixboxRuntime;

        impl JarRuntime for NixboxRuntime {
            fn run(&self, world: &str) -> anyhow::Result<()> {
                todo!()
            }
        }
    }
}
