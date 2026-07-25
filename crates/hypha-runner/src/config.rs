use std::env::{home_dir, var};
use std::path::PathBuf;
use std::str::FromStr;

use nix::unistd::getcwd;
use thiserror;

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
    state_dir: PathBuf,

    default_opts: Vec<String>,
}

impl RunnerConfig {
    pub fn get_state_dir(&self) -> &PathBuf {
        &self.state_dir
    }

    pub fn version_dir(&self, version: &str) -> Option<PathBuf> {
        match version {
            "latest" => Some(self.hytale_dir.join("install/release/package/game/latest/")),
            _ => None,
        }
    }
}

/// Linux (flatpak): .var/app.com.hypixel.HytaleLauncher/data/Hytale/ (or /opt/hytale for custom server installs)
/// macOS: Library/Application Support/Hytale/
/// Windows: %appdata%\Hytale\
fn default_root_game_dir() -> anyhow::Result<PathBuf> {
    let mut candidates = Vec::new();
    if cfg!(target_os = "linux") {
        candidates.push(PathBuf::from("/opt/hytale/"));
        candidates.push(
            home_dir()
                .expect("Failed to get home directory")
                .join(".var/app/com.hypixel.HytaleLauncher/data/Hytale/"),
        );
    }
    if cfg!(target_os = "windows") {
        candidates.push(PathBuf::from("%appdata%\\Hytale\\")); // TODO: fix
    }
    if cfg!(target_os = "macos") {
        candidates.push(PathBuf::from("Library/Application Support/Hytale/"));
    }

    candidates
        .into_iter()
        .find(|d| d.is_dir())
        .ok_or_else(|| anyhow::anyhow!("Failed to find default game directory"))
}

/// TODO: addded macos & windows support
fn default_state_dir() -> PathBuf {
    let home_dir = home_dir().expect("Failed to get home directory");
    let xdg_state_home = var("XDG_STATE_HOME")
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from(home_dir.join(".local/state")))
        .join("hypha-runner");

    let candidates = vec![
        #[cfg(target_os = "linux")]
        xdg_state_home,
    ];

    if let Some(dir) = candidates.iter().find(|d| d.is_dir()) {
        return dir.to_owned();
    } else {
        PathBuf::from(getcwd().expect("Failed to get working directory")).join("hypha-state")
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
                state_dir: default_state_dir(),
                default_opts: vec![],
            };
        }

        Self {
            hytale_dir: default_root_game_dir().expect("Failed to get game dir"),
            state_dir: default_state_dir(),
            default_opts: vec![],
        }
    }
}

impl FromStr for RunnerConfig {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let config = s.lines().fold(Self::default(), {
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

/// this helper load config from cwd_file -> xdg_file or default file
#[tracing::instrument]
pub(crate) fn load_config() -> anyhow::Result<RunnerConfig, RunnerConfigErrors> {
    fn try_load_from_file(path: &PathBuf) -> Option<RunnerConfig> {
        if !path.is_file() {
            return None;
        }
        let config_content = &std::fs::read_to_string(path).ok()?;
        RunnerConfig::from_str(&config_content).ok()
    }

    let cwd_config_file_path = getcwd().map_err(|err| RunnerConfigErrors::WorkInProgress {
        message: err.to_string(),
        span_trace: tracing_error::SpanTrace::capture(),
    })?;

    if let Some(config) = try_load_from_file(&cwd_config_file_path.join("hypha-runner.toml")) {
        return Ok(config);
    }

    let xdg_config_file_path = var("XDG_CONFIG_HOME")
        .ok()
        .map(|p| PathBuf::from(p).join("hypha-runner/hypha-runner.toml"))
        .filter(|p| p.is_file());

    if let Some(xdg_config_file) = xdg_config_file_path {
        if let Some(config) = try_load_from_file(&xdg_config_file) {
            return Ok(config);
        }
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
        let test_config = "\n[default]\r\nsometrash\nother_value = true\nHYTALE_DIR=/tmp/hytale";
        let config = RunnerConfig::from_str(test_config);
        assert!(config.is_ok(), "Failed to parse config");
        assert_eq!(config.unwrap().hytale_dir, PathBuf::from("/tmp/hytale"));
    }
}
