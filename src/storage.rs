use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// A session lap entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionLap {
    pub lap: u32,
    pub split: f64,
    pub total: f64,
}

/// A pomodoro cycle entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCycle {
    pub cycle: u32,
    #[serde(rename = "type")]
    pub cycle_type: String,
    pub duration: f64,
}

/// A recorded session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    #[serde(rename = "type")]
    pub session_type: String,
    pub started: String,
    pub ended: String,
    pub duration_seconds: f64,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub laps: Option<Vec<SessionLap>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycles: Option<Vec<SessionCycle>>,
}

/// Get the data directory path.
fn data_dir() -> PathBuf {
    if let Ok(home) = std::env::var("TERMINO_HOME") {
        return PathBuf::from(home);
    }
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".termino")
}

/// Get the sessions file path.
fn sessions_file() -> PathBuf {
    data_dir().join("sessions.json")
}

/// Load all sessions from file.
fn load_sessions() -> Vec<SessionData> {
    let path = sessions_file();
    if !path.exists() {
        return Vec::new();
    }
    match fs::read_to_string(&path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

/// Save sessions to file.
fn save_sessions(sessions: &[SessionData]) -> anyhow::Result<()> {
    let dir = data_dir();
    fs::create_dir_all(&dir)?;
    let content = serde_json::to_string_pretty(sessions)?;
    // Atomic write: write to temp, then rename
    let tmp_path = dir.join("sessions.json.tmp");
    fs::write(&tmp_path, &content)?;
    fs::rename(&tmp_path, sessions_file())?;
    Ok(())
}

/// Save a session to the log.
pub fn save_session(session: &SessionData) -> anyhow::Result<SessionData> {
    let mut sessions = load_sessions();
    sessions.push(session.clone());
    save_sessions(&sessions)?;
    Ok(session.clone())
}

/// Get sessions, optionally filtered by type and limited.
pub fn get_sessions(
    session_type: Option<&str>,
    limit: Option<usize>,
) -> anyhow::Result<Vec<SessionData>> {
    let mut sessions = load_sessions();
    if let Some(st) = session_type {
        sessions.retain(|s| s.session_type == st);
    }
    // Reverse so most recent first
    sessions.reverse();
    if let Some(limit) = limit {
        sessions.truncate(limit);
    }
    Ok(sessions)
}

/// Get total session count.
pub fn get_session_count() -> usize {
    load_sessions().len()
}
