use nix::unistd::getcwd;
use tracing::{Level, event, warn};

use clap::{Command, arg};

mod config;
mod runtimes;

use runtimes::Runtimes;

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
                .about("Supervisor for start and management hytale universe (servers).")
                .arg_required_else_help(true)
                .arg(arg!(<UNIVERSE> "UNIVERSE name to run").required(true))
                .arg(
                    arg!(--runtime <RUNTIME> "Runtime to use (container, nixbox)")
                        .value_parser(|s: &str| s.parse::<Runtimes>())
                        .default_value("container"),
                )
                .arg(arg!([OPTS] ..."Additional options to pass to the runtime (cgroups, mount, etc.) (WIP)")),
        )
        .subcommand(
            Command::new("list")
                .alias("ls")
                .about("List existing universes (WIP)"),
        )
        .get_matches();

    event!(Level::DEBUG, "Starting hypha-runner: {args:#?}");

    let config = config::load_config().expect("Can't load config");

    event!(Level::DEBUG, "Loaded config: {config:#?}");

    /// check if the server is already running or exists artifacts from previous run
    // let _ = ending_sanitize();
    let _ = init_state(&config);

    match &args.subcommand() {
        Some(("run", args)) => {
            let universes_dir = args
                .get_one::<String>("UNIVERSE")
                .expect("Universe name is required");
            let runtime = args
                .get_one::<Runtimes>("runtime")
                .expect("[Unexpected Error] Undefined runtime");

            // let runtime = runtime.run(&universe_dir).expect("Failed to run the runtime");

            // let _ = ending_sanitize();
        }
        Some(("list", _)) => {
            let universes_dir = config.get_state_dir().join("universes");

            println!("universes: {universes_dir:#?}");
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn init_state(config: &config::RunnerConfig) -> anyhow::Result<()> {
    warn!("[WIP] initialize directories and files");

    let state_dir = config.get_state_dir();

    if !state_dir.exists() {
        std::fs::create_dir_all(&state_dir).expect("Failed to create working directory");
    }

    Ok(())
}
