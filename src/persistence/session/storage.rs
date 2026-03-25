use super::dto::AppSession;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub fn load_session() -> Option<AppSession> {
    let path = session_file_path()?;
    let text = fs::read_to_string(path).ok()?;
    serde_json::from_str(&text).ok()
}

pub fn save_session(session: &AppSession) -> Result<(), String> {
    let path = session_file_path().ok_or_else(|| "session dir unavailable".to_string())?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| err.to_string())?;
    }
    let text = serde_json::to_string_pretty(session).map_err(|err| err.to_string())?;
    fs::write(path, text).map_err(|err| err.to_string())
}

fn session_file_path() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("rs", "gen", "pixel_paint_rs")?;
    Some(dirs.data_local_dir().join("session.json"))
}
