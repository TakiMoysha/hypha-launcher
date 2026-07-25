use std::env::{home_dir, var};
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::bail;
use nix::unistd::getcwd;
use thiserror;

pub const GAME_POSTFIX: &str = "Hytale/install/release/package/game/latest/";

#[derive(Debug, thiserror::Error)]
pub enum RunnerConfigErrors {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("WIP Error: {message}")]
    WorkInProgress {
        message: String,
        span_trace: tracing_error::SpanTrace,
    },
}

#[derive(Debug)]
pub struct RunnerConfig {
    hytale_dir: PathBuf,
    default_opts: Vec<String>,
}

/// Linux: /Hytale/ (or /opt/hytale for custom server installs)
/// macOS: ~/Library/Application Support/Hytale/
/// Windows: %appdata%\Hytale\
fn default_root_game_dir() -> anyhow::Result<PathBuf> {
    let candidates = vec![
        #[cfg(target_os = "windows")]
        PathBuf::from("%appdata%\\Hytale\\"), // TODO: fix this
        #[cfg(target_os = "macos")]
        PathBuf::from("/Library/Application Support/Hytale/"),
        #[cfg(target_os = "linux")]
        home_dir()
            .expect("Failed to get home directory")
            .join(".var/app/com.hypixel.HytaleLauncher/data/Hytale/"),
        #[cfg(target_os = "linux")]
        PathBuf::from("/opt/hytale/"),
    ]
    .into_iter()
    .filter(|dir| dir.is_dir())
    .collect::<Vec<PathBuf>>();

    if candidates.first().is_some() {
        Ok(candidates.first().unwrap().to_owned())
    } else {
        bail!("Failed to find default game directory")
    }
}

impl Default for RunnerConfig {
    fn default() -> Self {
        #[cfg(debug_assertions)]
        if let Ok(p) = var("DEV__HYTALE_DIR") {
            let dir = PathBuf::from(p);
            if !dir.is_dir() {
                panic!("DEV__HYTALE_DIR does not exist");
            }

            return RunnerConfig {
                hytale_dir: dir,
                default_opts: vec![],
            };
        }

        Self {
            hytale_dir: default_root_game_dir().expect("Failed to get game dir"),
            default_opts: vec![],
        }
    }
}

impl FromStr for RunnerConfig {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let config = s.split('\n').fold(Self::default(), {
            |mut acc, line| {
                if let Some((key, value)) = line.split_once('=') {
                    match key {
                        "HYTALE_DIR" => acc.hytale_dir = PathBuf::from(value),
                        _ => acc.default_opts.push(line.to_string()),
                    }
                }
                acc
            }
        });

        Ok(config)
    }
}

/// helper, loaded config file, default config, args from cli and merge them
/// TODO: add config dir with xdg or getcwd
#[tracing::instrument]
pub(crate) fn load_config() -> anyhow::Result<RunnerConfig, RunnerConfigErrors> {
    let cwd_config_file_path = getcwd()
        .map_err(|err| RunnerConfigErrors::WorkInProgress {
            message: err.to_string(),
            span_trace: tracing_error::SpanTrace::capture(),
        })?
        .join("hypha-runner.toml");

    if cwd_config_file_path.is_file() {
        let config_content = &std::fs::read_to_string(cwd_config_file_path)?;

        return Ok(RunnerConfig::from_str(config_content)?);
    }

    let raw_xdg_config_file_path =
        var("XDG_CONFIG_HOME").map_err(|err| RunnerConfigErrors::WorkInProgress {
            message: err.to_string(),
            span_trace: tracing_error::SpanTrace::capture(),
        })?;
    let xdg_config_file_path =
        PathBuf::from(raw_xdg_config_file_path).join("hypha-runner/hypha-runner.toml");

    if xdg_config_file_path.is_file() {
        let config_content = &std::fs::read_to_string(xdg_config_file_path)?;

        return Ok(RunnerConfig::from_str(config_content)?);
    }

    Ok(RunnerConfig::default())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn should_return_root_path_by_planform() {
        let root_game_dir = default_root_game_dir();
        assert!(root_game_dir.is_ok(), "Failed to get root game dir");
    }

    #[test]
    fn should_pase_config_from_string() {
        let config = RunnerConfig::from_str("HYTALE_DIR=/tmp/hytale");
        assert!(config.is_ok(), "Failed to parse config");
    }
}
