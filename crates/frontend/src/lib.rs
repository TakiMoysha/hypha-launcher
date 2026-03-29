use hypha_backend::add;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
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
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
