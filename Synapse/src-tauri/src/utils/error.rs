//! Error types and utilities
//! 
//! This module provides common error types and utilities used throughout the application.

use std::fmt;
use serde::{Serialize, Deserialize};

/// Result type alias using AppError
pub type AppResult<T> = Result<T, AppError>;

/// Application error types
#[derive(Debug, Serialize, Deserialize)]
pub enum AppError {
    /// Invalid input or parameters
    InvalidInput(String),
    /// Resource not found
    NotFound(String),
    /// Permission denied
    PermissionDenied(String),
    /// Internal error
    Internal(String),
    /// Network or IO error
    Network(String),
    /// API error
    Api(String),
}

impl AppError {
    /// Creates a new invalid input error
    pub fn invalid_input<S: Into<String>>(msg: S) -> Self {
        Self::InvalidInput(msg.into())
    }

    /// Creates a new not found error
    pub fn not_found<S: Into<String>>(msg: S) -> Self {
        Self::NotFound(msg.into())
    }

    /// Creates a new permission denied error
    pub fn permission_denied<S: Into<String>>(msg: S) -> Self {
        Self::PermissionDenied(msg.into())
    }

    /// Creates a new internal error
    pub fn internal<S: Into<String>>(msg: S) -> Self {
        Self::Internal(msg.into())
    }

    /// Creates a new network error
    pub fn network<S: Into<String>>(msg: S) -> Self {
        Self::Network(msg.into())
    }

    /// Creates a new API error
    pub fn api<S: Into<String>>(msg: S) -> Self {
        Self::Api(msg.into())
    }
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Self::NotFound(msg) => write!(f, "Not found: {}", msg),
            Self::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            Self::Internal(msg) => write!(f, "Internal error: {}", msg),
            Self::Network(msg) => write!(f, "Network error: {}", msg),
            Self::Api(msg) => write!(f, "API error: {}", msg),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::Internal(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        Self::Network(err.to_string())
    }
} 