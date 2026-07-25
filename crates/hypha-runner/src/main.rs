use std::{path::PathBuf, str::FromStr};

use nix::unistd::getcwd;
use tracing::{Level, error, event, info, warn};

use clap::{Command, arg};

use crate::runtimes::JarRuntime;

use self::runtimes::Runtimes;

mod config;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Command::new("hypha-runner")
        .version("0.1.0")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("run")
                .about("Supervisor for start and management hytale worlds (servers).")
                .arg_required_else_help(true)
                .arg(arg!(<WORLD> "World name to run").required(true))
                .arg(
                    arg!(--runtime <RUNTIME> "Runtime to use (container, nixbox)")
                        .value_parser(|s: &str| s.parse::<runtimes::Runtimes>())
                        .default_value("container"),
                )
                .arg(arg!([OPTS] ..."Additional options to pass to the runtime (cgroups, mount, etc.) (WIP)")),
        )
        .subcommand(
            Command::new("list")
                .alias("ls")
                .about("List existing worlds (WIP)"),
        )
        .get_matches();

    event!(Level::DEBUG, "Starting hypha-runner: {args:#?}");

    let config = config::load_config();

    event!(Level::DEBUG, "Loaded config: {config:#?}");

    let _ = ending_sanitize();

    match &args.subcommand() {
        Some(("run", args)) => {
            let world = args
                .get_one::<String>("WORLD")
                .expect("World name is required");
            let runtime = args
                .get_one::<Runtimes>("runtime")
                .expect("[Unexpected Error] Undefined runtime");

            let runtime = runtime.run(&world).expect("Failed to run the runtime");
        }
        Some(("list", _)) => {
            let wrk_dir = getcwd().expect("Failed to get working directory");
        }
        _ => unreachable!(),
    }

    Ok(())
}

/// check if the server is already running or exists artifacts from previous run
fn ending_sanitize() -> anyhow::Result<()> {
    warn!("[WIP] check if the server is already running or artifacts from previous run exists");
    Ok(())
}

fn init_state() -> anyhow::Result<()> {
    warn!("[WIP] initialize directories and files");
    let wrk_dir =
        PathBuf::from(getcwd().expect("Failed to get working directory")).join("hypha-workdir");

    if !wrk_dir.exists() {
        std::fs::create_dir_all(&wrk_dir).expect("Failed to create working directory");
    }

    Ok(())
}

mod runtimes {
    use std::str::FromStr;

    use container_runtime::ContainerRuntime;

    #[cfg(target_os = "linux")]
    use nixbox_runtime::NixboxRuntime;

    use anyhow::anyhow;
    pub trait JarRuntime {
        fn run(&self, world: &str) -> anyhow::Result<()>;
        fn clean(&self, world: &str) -> anyhow::Result<()>;
    }

    // ==========================================================================================
    pub fn gen_cgroup_name(name: &str) -> String {
        format!("hypha-cgroup-{name}")
    }
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

    mod container_runtime {
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
    }

    mod nixbox_runtime {
        use std::ffi::CString;
        use std::fs;
        use std::path::PathBuf;

        use nix::{
            mount::{MsFlags, mount},
            sys::stat::{Mode, SFlag, mknod},
            sys::wait::waitpid,
            unistd::{Gid, Uid},
        };

        use super::{JarRuntime, gen_cgroup_name};

        #[derive(thiserror::Error, Debug)]
        pub enum NixboxRuntimeErrors {
            #[error("IO error: {0}")]
            Io(#[from] std::io::Error),
            #[error("Setup Error: {0}")]
            Setup(#[from] nix::Error),
            #[error(transparent)]
            Any(#[from] Box<dyn std::error::Error + Send + Sync>),
        }

        #[derive(Clone)]
        pub struct CgroupLimitsOpts {
            max_memory: u32,
            max_pids: u32,
        }

        #[derive(Clone)]
        pub struct NixboxRuntime {
            id: String,
            // child pid, not supervisor
            pid: Option<nix::unistd::Pid>,
            container_dir: Option<PathBuf>,
            cgroup_path: Option<PathBuf>,
            cgroup_limits: CgroupLimitsOpts,
        }

        impl Default for NixboxRuntime {
            fn default() -> Self {
                Self {
                    id: "default".to_string(),
                    pid: None,
                    container_dir: Some(PathBuf::from("")),
                    cgroup_path: Some(
                        PathBuf::from("/sys/fs/cgroup").join(gen_cgroup_name(&"default")),
                    ),
                    cgroup_limits: CgroupLimitsOpts {
                        max_memory: 2048,
                        max_pids: 1024,
                    },
                }
            }
        }

        impl NixboxRuntime {
            pub fn set_host_name(&self) -> nix::Result<()> {
                nix::unistd::sethostname("hypha-nixbox")
            }

            /// /proc - system info
            pub fn mount_proc(&self) -> nix::Result<()> {
                mount(
                    Some("proc"),
                    "/proc",
                    Some("proc"),
                    MsFlags::empty(),
                    None::<&str>,
                )?;
                Ok(())
            }

            /// number take from https://www.kernel.org/doc/Documentation/admin-guide/devices.txt
            /// devices that are needed for the server to work
            pub fn mount_dev(&self) -> nix::Result<()> {
                // mount tmpfs as /dev (https://www.kernel.org/doc/Documentation/filesystems/tmpfs.txt)
                // runtime-only directory, don't touch real /dev
                mount(
                    Some("dev"),
                    "/dev",
                    Some("tmpfs"),
                    MsFlags::empty(),
                    None::<&str>,
                )?;

                // null - empty device (void)
                // mkdev генерирует номер для устройства: мажорный, минорый
                let null = nix::sys::stat::makedev(1, 3);

                // CharacterDevice с правами 0o666 (rw-rw-rw-) https://man7.org/linux/man-pages/man2/mknod.2.html
                mknod(
                    CString::new("/dev/null").unwrap().as_c_str(),
                    SFlag::S_IFCHR,
                    Mode::from_bits_truncate(0o666),
                    null,
                )?;

                // tty - terminal output (server checks for output)
                let tty = nix::sys::stat::makedev(5, 0);

                mknod(
                    CString::new("/dev/tty").unwrap().as_c_str(),
                    SFlag::S_IFCHR,
                    Mode::from_bits_truncate(0o666),
                    tty,
                )?;

                // urandom - generate random numbers (UUID) (https://man7.org/linux/man-pages/man7/random.7.html)
                let urandom = nix::sys::stat::makedev(1, 9); // Магические числа для urandom
                mknod(
                    CString::new("/dev/urandom").unwrap().as_c_str(),
                    SFlag::S_IFCHR,
                    Mode::from_bits_truncate(0o666),
                    urandom,
                )?;

                // `/dev/zero` - zero-generator (https://man7.org/linux/man-pages/man7/random.7.html)
                let zero = nix::sys::stat::makedev(1, 5);
                mknod(
                    CString::new("/dev/zero").unwrap().as_c_str(),
                    SFlag::S_IFCHR,
                    Mode::from_bits_truncate(0o666),
                    zero,
                )?;

                Ok(())
            }

            pub fn setup_fs(&self) -> Result<(), NixboxRuntimeErrors> {
                mount(
                    None::<&str>,
                    "/",
                    None::<&str>,
                    MsFlags::MS_REC | MsFlags::MS_PRIVATE,
                    None::<&str>,
                )
                .unwrap();

                self.mount_proc()?;
                self.mount_dev()?;
                self.set_host_name()?;

                // reset permissions to user or unshare
                // nix::unistd::setgid(Gid::from_raw(1001)).expect("Failed to set GID");
                // nix::unistd::setuid(Uid::from_raw(1001)).expect("Failed to set UID");

                Ok(())
            }

            pub fn setup_cgroups(&self) -> Result<(), NixboxRuntimeErrors> {
                let id = &self.id;
                let cgroup = PathBuf::from("/sys/fs/cgroup").join(gen_cgroup_name(id));
                fs::create_dir_all(&cgroup)?;

                fs::write(
                    cgroup.join("memory.max"),
                    (self.cgroup_limits.max_memory).to_string(),
                )?;
                fs::write(
                    cgroup.join("pids.max"),
                    self.cgroup_limits.max_pids.to_string(),
                )?;

                // TODO: cgrpus.procs required pid of the working process, not a supervisor/parent
                // let pid = std::process::id(); // parent pid
                // fs::write(cgroup.join("cgroup.procs"), pid.to_string())?;

                Ok(())
            }

            // TODO:
            // - unshare so that the process runs in its own namespace (https://man7.org/linux/man-pages/man1/unshare.1.html)
            //unshare(
            //     CloneFlags::CLONE_NEWNS   | //
            //     CloneFlags::CLONE_NEWPID  | //
            //     CloneFlags::CLONE_NEWNET  | //
            //     CloneFlags::CLONE_NEWIPC    // запрет IPC (https://man7.org/linux/man-pages/man2/clone.2.html)
            // )?;
            // pub fn spawn_supervisor(&self) -> Result<()> {}
        }

        impl Drop for NixboxRuntime {
            fn drop(&mut self) {
                if let Some(pid) = self.pid {
                    println!("TODO, nixbox runtime destructor, PID {:?}", pid);

                    let _ = nix::sys::signal::kill(pid, nix::sys::signal::SIGTERM);

                    for _ in 0..50 {
                        match waitpid(pid, Some(nix::sys::wait::WaitPidFlag::WNOHANG)) {
                            Ok(_) => break,
                            _ => std::thread::sleep(std::time::Duration::from_millis(100)),
                        }
                    }

                    let _ = nix::sys::signal::kill(pid, nix::sys::signal::SIGKILL);
                    let _ = waitpid(pid, None);
                }

                // cleanup cgroup file
                if let Some(path) = &self.cgroup_path {
                    path.exists().then(|| {
                        fs::remove_dir(path).expect("[ERROR] Failed to remove cgroup dir");
                    });
                };

                // cleanup temporary server files (upper layer - ?)
                // if self.upper_path.exists() {
                //     let _ = fs::remove_dir_all(&self.upper_path);
                //     let _ = fs::remove_dir_all(&self.work_path);
                //     println!("[INFO] Upper layer and work directories cleaned.");
                // }

                // merged dir only mount point, linux kernel will unmount it
                if let Some(path) = &self.cgroup_path {
                    path.exists().then(|| {
                        fs::remove_dir_all(path).expect("[ERROR] Failed to remove cgroup dir")
                    });
                }

                if let Some(path) = &self.container_dir {
                    path.exists().then(|| {
                        fs::remove_dir_all(path).expect("[ERROR] Failed to remove container dir")
                    });
                }
            }
        }

        impl JarRuntime for NixboxRuntime {
            fn run(&self, world: &str) -> anyhow::Result<()> {
                let mut server_process = tokio::process::Command::new("java");

                Ok(())
            }

            fn clean(&self, world: &str) -> anyhow::Result<()> {
                // see drop trait
                todo!()
            }
        }
    }
}
