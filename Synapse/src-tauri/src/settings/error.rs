use thiserror::Error;

/// Error types for settings operations
#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Failed to access config directory")]
    ConfigDirNotFound,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Keyring error: {0}")]
    Keyring(#[from] keyring::Error),
}

// We'll add more error variants and user-friendly messages as needed when we implement:
// - Settings validation
// - Settings migration
// - Concurrent access handling
// - Value validation 