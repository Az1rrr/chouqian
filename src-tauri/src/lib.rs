use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoryEntry {
    pub winner: String,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    #[serde(default)]
    pub names: Vec<String>,
    #[serde(default)]
    pub history: Vec<HistoryEntry>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            names: vec![],
            history: vec![],
        }
    }
}

fn config_path() -> PathBuf {
    let exe = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    let exe_dir = exe.parent().unwrap_or_else(|| std::path::Path::new("."));

    if exe_dir.to_string_lossy().contains("target") {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("config.json")
    } else {
        exe_dir.join("config.json")
    }
}

fn read_config_file() -> AppConfig {
    let path = config_path();
    if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(AppConfig::default)
    } else {
        AppConfig::default()
    }
}

fn write_config_file(config: &AppConfig) -> Result<(), String> {
    let path = config_path();
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, json).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_config() -> Result<AppConfig, String> {
    Ok(read_config_file())
}

#[tauri::command]
fn write_config(names: Vec<String>, history: Vec<HistoryEntry>) -> Result<(), String> {
    let config = AppConfig { names, history };
    write_config_file(&config)
}

#[tauri::command]
fn clear_history() -> Result<(), String> {
    let mut config = read_config_file();
    config.history = vec![];
    write_config_file(&config)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_config, write_config, clear_history])
        .run(tauri::generate_context!())
        .expect("启动失败");
}
