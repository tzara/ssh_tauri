use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State};

mod config;
mod models;
mod ssh;

use models::Config;

pub struct AppState {
    pub config: Mutex<Config>,
}

#[tauri::command]
fn get_config(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let config = state.config.lock().map_err(|e| e.to_string())?;

    let servers: Vec<serde_json::Value> = config
        .servers
        .iter()
        .map(|(k, v)| serde_json::json!({ "key": k, "host": v.host }))
        .collect();

    let commands: Vec<serde_json::Value> = config
        .commands
        .iter()
        .map(|(k, v)| {
            serde_json::json!({
                "key":  k,
                "name": v.name,
                "desc": v.desc,
                "cmd":  v.cmd,
            })
        })
        .collect();

    Ok(serde_json::json!({ "servers": servers, "commands": commands }))
}

#[tauri::command]
async fn execute_command(app: AppHandle, host: String, command: String) -> Result<(), String> {
    std::thread::spawn(move || {
        let _ = app.emit("ssh-started", ());

        match ssh::execute_ssh(&host, &command) {
            Ok(output) => {
                let _ = app.emit("ssh-output", output);
            }
            Err(e) => {
                let _ = app.emit("ssh-error", e.to_string());
            }
        }

        let _ = app.emit("ssh-finished", ());
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_config, execute_command])
        .setup(|app| {
            let cfg = config::load_config(&app.handle());
            app.manage(AppState {
                config: Mutex::new(cfg),
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
