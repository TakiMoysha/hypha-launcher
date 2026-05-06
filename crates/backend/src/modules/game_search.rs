use std::collections::HashMap;
use std::env::consts;
use std::process::Command;

use anyhow::{anyhow, bail};
use serde::{Deserialize, Serialize};

/// Represents a discovered game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredGame {
    pub name: String,
    pub executable_path: String,
    pub platform: String, // flatpak, steam, etc.
    pub app_id: String,   // flatpak app ID, steam app ID, etc.
    pub description: Option<String>,
    pub version: Option<String>,
}

/// Platform-specific game search traits
pub trait GameSearch: Send + Sync {
    fn search_games(&self) -> Vec<DiscoveredGame>;
}

/// Linux Flatpak game search implementation
pub struct FlatpakGameSearch;
/// TODO: tests
///     - if flatpak (not)installed
///     - game is (not)installed
pub fn flatpak_is_installed_hytale() -> anyhow::Result<()> {
    struct DiscoveredGame {
        name: String,
        application: String,
        version: String,
        comment: String,
    }

    let output = Command::new("flatpak")
        .arg("list")
        .arg("--app")
        .arg("--columns=name,application,version")
        .output()
        .map_err(|e| anyhow!("Can't execute flatpak: {}", e))?;

    if !output.status.success() {
        bail!("Failed to list flatpak apps: {}", output.status);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    // let stdout = String::from_utf8_lossy(&output.stdout);
    // for line in stdout.lines() {
    //     let fields: Vec<&str> = line.split(',').collect();

    todo!()
}

impl GameSearch for FlatpakGameSearch {
    fn search_games(&self) -> Vec<DiscoveredGame> {
        let _ = flatpak_is_installed_hytale();
        todo!();

        // match output {
        //     Ok(output) if output.status.success() => {
        //
        //         let stdout = String::from_utf8_lossy(&output.stdout);
        //         for line in stdout.lines() {
        //                 };
        //
        //                 // Try to find the executable path for Flatpak apps
        //                 let executable_path = Self::find_flatpak_executable(app_id);
        //
        //                 games.push(DiscoveredGame {
        //                     name: name.to_string(),
        //                     executable_path: executable_path
        //                         .unwrap_or_else(|| format!("flatpak run {}", app_id)),
        //                     platform: "flatpak".to_string(),
        //                     app_id: app_id.to_string(),
        //                     description,
        //                     version,
        //                 });
        //             }
        //         }
        //     }
        //     Err(e) => {
        //         eprintln!("Failed to list flatpak apps: {}", e);
        //     }
        //     _ => {
        //         eprintln!("Flatpak command failed");
        //     }
        // }
        //
        // games
    }
}

impl FlatpakGameSearch {
    fn find_flatpak_executable(&self, app_id: &str) -> Option<String> {
        // Try to get the executable path from flatpak info
        let output = Command::new("flatpak")
            .arg("info")
            .arg(app_id)
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        // Look for the "Location:" line which contains the installation path
        for line in stdout.lines() {
            if line.starts_with("Location:") {
                let path = line["Location:".len()..].trim();
                // Construct potential executable path
                // Flatpak apps usually have executables in /app/bin/ inside the installation
                let exe_path = format!("{}/bin/{}", path, app_id.replace('.', "-"));
                if std::path::Path::new(&exe_path).exists() {
                    return Some(exe_path);
                }
                // Fallback to the flatpak run command
                return Some(format!("flatpak run {}", app_id));
            }
        }

        Some(format!("flatpak run {}", app_id))
    }
}

// impl PlatformGameSearch {
//     pub fn create() -> Box<dyn GameSearch> {
//         match consts::OS {
//             "linux" => {
//                 // For now, we'll focus on Flatpak as requested
//                 Box::new(FlatpakGameSearch {})
//             }
//             "windows" => {
//                 // TODO: Implement Windows search (registry, Steam, etc.)
//                 eprintln!("Windows game search not yet implemented");
//                 Box::new(FlatpakGameSearch {}) // Placeholder
//             }
//             "macos" => {
//                 // TODO: Implement macOS search (App Store, Steam, etc.)
//                 eprintln!("macOS game search not yet implemented");
//                 Box::new(FlatpakGameSearch {}) // Placeholder
//             }
//             _ => {
//                 eprintln!("Unsupported platform: {}", consts::OS);
//                 Box::new(FlatpakGameSearch {}) // Fallback
//             }
//         }
//     }
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_exec_flatpak_and_read_output() {
        let output = Command::new("flatpak")
            .arg("list")
            .arg("--app")
            .arg("--columns=name,application,version,comment")
            .output();

        assert!(output.is_ok(), "required installed flatpak");
    }

    #[test]
    fn should_fail_when_flatpak_not_installed() {
        let output = Command::new("flatpak")
            .arg("list")
            .arg("--app")
            .arg("--columns=name,application,version")
            .output()
            .map_err(|e| anyhow!("Can't execute flatpak: {}", e));

        if let Err(output) = output {
            assert_eq!(output.to_string(), "Can't execute flatpak: No such file or directory (os error 2)");
        }
    }
}


