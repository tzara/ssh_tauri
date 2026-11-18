use crate::models::Config;
use std::path::{Path, PathBuf};
use tauri::Manager;

pub fn load_config(app: &tauri::AppHandle) -> Config {
    let path = resolve_hosts_toml(app);
    let contents = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("hosts.toml read failed ({}): {e}", path.display()));
    toml::from_str(&contents).expect("hosts.toml parse failed")
}

fn resolve_hosts_toml(app: &tauri::AppHandle) -> PathBuf {
    // 1. resource dir (bundled release)
    if let Ok(dir) = app.path().resource_dir() {
        let p = dir.join("hosts.toml");
        if p.exists() { return p; }
    }
    // 2. exe dir (dev / portable)
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("hosts.toml");
            if p.exists() { return p; }
        }
    }
    // 3. cwd fallback
    Path::new("hosts.toml").to_path_buf()
}
