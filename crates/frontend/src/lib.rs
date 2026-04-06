use hypha_backend::add;
use tauri::Manager;

pub mod spawner {
    use std::collections::VecDeque;
    use std::io::BufReader;
    use std::process::{Command, Stdio};
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    pub struct LogEntry {
        pub raw: String,
    }

    #[derive(Debug)]
    pub struct LogCollector {
        logs: Arc<Mutex<VecDeque<LogEntry>>>,
        max_logs: usize,
    }

    #[tauri::command]
    pub fn spawn_hytale_server(
        process: String,
        args: Vec<String>,
        log_callback: tauri::State<'_, LogCollector>,
    ) {
        let mut child = Command::new(process)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute process");

        let stdout = child.stdout.expect("failed to get stdout");
        let stderr = child.stderr.expect("failed to get stderr");

        let stdout_reader = BufReader::new(stdout);
        let stderr_reader = BufReader::new(stderr);
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greeting_format(name: &str) -> String {
    format!(
        "Hello, {}! You've been greeted from Rust! Result: {}",
        name,
        add(1, 8)
    )
}

struct Authentication;

struct ModPresenter {
    id: String,
    modname: String,
    version: String,
}

struct AppData {
    modlist: Vec<ModPresenter>,
    auth: Authentication,
}
impl Default for AppData {
    fn default() -> Self {
        Self {
            modlist: vec![ModPresenter {
                id: String::from("TestModifier-0.1.0-test"),
                modname: String::from("TestModifier"),
                version: String::from("0.1.0-test"),
            }],
            auth: Authentication,
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppData::default());
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![spawner::spawn_hytale_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
