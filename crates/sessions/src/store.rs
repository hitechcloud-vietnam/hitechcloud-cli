//! Session store with SQLite backend

use crate::session::Session;
use hitechcloud_core::Message;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Session store error
#[derive(Debug, thiserror::Error)]
pub enum SessionStoreError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Session not found: {0}")]
    NotFound(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Result type for session store operations
pub type SessionStoreResult<T> = std::result::Result<T, SessionStoreError>;

/// Session store with SQLite backend
pub struct SessionStore {
    db_path: std::path::PathBuf,
}

impl SessionStore {
    /// Create a new SessionStore
    pub fn new(data_dir: &str) -> SessionStoreResult<Self> {
        let db_path = std::path::PathBuf::from(data_dir).join("sessions.db");

        // Create directory if it doesn't exist
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let store = Self { db_path };
        store.init_db()?;
        Ok(store)
    }

    /// Initialize the database
    fn init_db(&self) -> SessionStoreResult<()> {
        // For now, use a simple file-based approach
        // TODO: Implement proper SQLite with rusqlite
        Ok(())
    }

    /// Save a session
    pub fn save(&self, session: &Session) -> SessionStoreResult<()> {
        let sessions_dir = self.db_path.parent().unwrap().join("sessions");
        std::fs::create_dir_all(&sessions_dir)?;

        let file_path = sessions_dir.join(format!("{}.json", session.id));
        let content = serde_json::to_string_pretty(session)?;
        std::fs::write(file_path, content)?;

        Ok(())
    }

    /// Load a session by ID
    pub fn load(&self, id: Uuid) -> SessionStoreResult<Option<Session>> {
        let sessions_dir = self.db_path.parent().unwrap().join("sessions");
        let file_path = sessions_dir.join(format!("{}.json", id));

        if file_path.exists() {
            let content = std::fs::read_to_string(file_path)?;
            let session: Session = serde_json::from_str(&content)?;
            Ok(Some(session))
        } else {
            Ok(None)
        }
    }

    /// List all sessions
    pub fn list(&self) -> SessionStoreResult<Vec<Session>> {
        let sessions_dir = self.db_path.parent().unwrap().join("sessions");

        if !sessions_dir.exists() {
            return Ok(vec![]);
        }

        let mut sessions = Vec::new();

        for entry in std::fs::read_dir(&sessions_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    if let Ok(session) = serde_json::from_str::<Session>(&content) {
                        sessions.push(session);
                    }
                }
            }
        }

        // Sort by last activity (newest first)
        sessions.sort_by(|a, b| b.last_activity.cmp(&a.last_activity));

        Ok(sessions)
    }

    /// Delete a session
    pub fn delete(&self, id: Uuid) -> SessionStoreResult<()> {
        let sessions_dir = self.db_path.parent().unwrap().join("sessions");
        let file_path = sessions_dir.join(format!("{}.json", id));

        if file_path.exists() {
            std::fs::remove_file(file_path)?;
        }

        Ok(())
    }
}
