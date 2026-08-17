use std::collections::HashMap;

use tracing::{debug, info, warn};

use clap::{Command, arg, value_parser};

mod config;
mod runtimes;

use runtimes::Runtimes;

use clap_complete::aot::{Shell, generate};

fn build_cli() -> Command {
    Command::new("hypha-runner")
        .version("0.1.0")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(
            Command::new("run")
                .about("Supervisor for start and management hytale universe (servers).")
                .arg_required_else_help(true)
                .arg(arg!(<UNIVERSE> "Name of Universe (server) to run").required(true))
                .arg(
                    arg!(--runtime <RUNTIME> "One of: bare, container, nixbox.")
                        .help("Container use docker/podman, nixbox is linux sandbox, bare - no container")
                        .value_parser(|s: &str| s.parse::<Runtimes>())
                        .default_value("bare"),
                )
                .arg(
                    arg!(--assets [ASSETS_PATH] "(WIP) Custom path to assets path (archive or directory).")
                        .value_hint(clap::ValueHint::AnyPath) // archive or directory
                )
                .arg(arg!([OPTS] ..."(WIP) Additional options to pass to the runtime (cgroups, mount, etc.) ")),
        )
        .subcommand(
            Command::new("healthcheck")
                .about("WIP: Validate universes, see backups, check required apps, etc")
        )
        .subcommand(
            Command::new("list")
                .alias("ls")
                .about("List existing universes"),
        )
        .subcommand(
            Command::new("config")
                .about("Print configuration file"),
        )
        .subcommand(
            Command::new("completions")
                .about("Generate shell completions")
                .arg(arg!(<SHELL> "Shell to generate completions for").value_parser(value_parser!(Shell))))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .compact()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = build_cli().get_matches();

    debug!("Starting hypha-runner in directory: {args:?}");

    let config = config::load_config_with_autodiscovery().expect("Can't load config");

    // TODO: check if the server is already running or exists artifacts from previous run
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
            let _assets_dir = args.get_one::<std::path::PathBuf>("assets").cloned();

            runtime
                .run(universe_name, &config)
                .await
                .expect("Something went wrong in runtime");
        }

        Some(("healthcheck", _)) => {
            todo!()
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
        Some(("completions", sub_args)) => {
            let shell = sub_args
                .get_one::<Shell>("SHELL")
                .copied()
                .expect("Shell is required");
            let mut cmd = build_cli();
            generate(shell, &mut cmd, "hypha-runner", &mut std::io::stdout());
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
    if !config.work_dir.exists() {
        std::fs::create_dir_all(&config.work_dir).expect("Failed to create state directory");
        info!("Created state directory <{}>", config.work_dir.display());
    }

    Ok(())
}
