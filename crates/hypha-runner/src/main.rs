use std::collections::HashMap;

use nix::unistd::getcwd;
use tracing::{Level, debug, debug_span, info, warn};

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
                .arg(arg!([ASSETS_PATH] "(WIP) Custom path to assets path or use default from server version."))
                .arg(arg!([OPTS] ..."(WIP) Additional options to pass to the runtime (cgroups, mount, etc.) ")),
        )
        .subcommand(
            Command::new("list")
                .alias("ls")
                .about("List existing universes (WIP)"),
        )
        .subcommand(
            Command::new("config")
                .about("Print configuration file (WIP)"),
        )
        .get_matches();

    debug!("Starting hypha-runner in directory: {args:?}");

    let config = config::load_config().expect("Can't load config");

    /// check if the server is already running or exists artifacts from previous run
    // let _ = ending_sanitize();
    let _ = init_state(&config);

    match &args.subcommand() {
        Some(("run", args)) => {
            let universe_name = args
                .get_one::<String>("UNIVERSE")
                .expect("Universe name is required");
            let runtime = args
                .get_one::<Runtimes>("runtime")
                .expect("[Unexpected Error] Undefined runtime");

            // let runtime = runtime
            //     .run(&universe_dir)
            //     .expect("Failed to run the runtime");

            // let _ = ending_sanitize();
        }
        Some(("list", _)) => {
            let universes_dir = config.get_universes_dir_path();
            let universes = get_list_universes(&universes_dir);

            if universes.is_empty() {
                println!("No universes found");
                return Ok(());
            } else {
                println!("Existing universes:");
                for (name, path) in universes {
                    println!("\t{name}: {path:?}");
                }
            }
        }
        Some(("config", _)) => {
            println!("{config:#?}");
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn get_list_universes(universes_dir: &std::path::Path) -> HashMap<String, std::path::PathBuf> {
    let mut universes = HashMap::new();

    debug!("Reading universes dir: {universes_dir:?}");
    match std::fs::read_dir(&universes_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                debug!("Entry: {entry:?}");
                if entry.file_type().is_ok_and(|t| t.is_dir()) {
                    universes.insert(
                        entry.file_name().to_string_lossy().to_string(),
                        entry.path(),
                    );
                }
            }
        }
        Err(e) => {
            warn!("Could not read universes dir: {e}");
        }
    }

    universes
}

fn init_state(config: &config::RunnerConfig) -> anyhow::Result<()> {
    if !config.state_dir.exists() {
        std::fs::create_dir_all(&config.state_dir).expect("Failed to create state directory");
        info!("Created state directory <{}>", config.state_dir.display());
    }

    Ok(())
}
