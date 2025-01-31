//! Chat Service module
//! 
//! This module handles chat session management and interactions.
//! It provides functionality for creating, managing, and persisting chat sessions.

use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::utils::AppResult;
use super::ai::{AIProvider, Message};

/// Represents a chat session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    /// Unique identifier for the session
    pub id: String,
    /// Title of the chat session
    pub title: String,
    /// Messages in the session
    pub messages: Vec<Message>,
    /// When the session was created
    pub created_at: DateTime<Utc>,
    /// When the session was last updated
    pub updated_at: DateTime<Utc>,
}

/// Manages chat sessions and interactions with AI providers
#[derive(Debug)]
pub struct ChatManager {
    /// The active AI provider
    provider: Arc<RwLock<Option<Arc<dyn AIProvider>>>>,
    /// Active chat sessions
    sessions: Arc<RwLock<Vec<ChatSession>>>,
}

impl ChatManager {
    /// Creates a new ChatManager instance
    pub fn new() -> Self {
        Self {
            provider: Arc::new(RwLock::new(None)),
            sessions: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Sets the active AI provider
    pub async fn set_provider(&self, provider: Arc<dyn AIProvider>) {
        let mut provider_lock = self.provider.write().await;
        *provider_lock = Some(provider);
    }

    /// Creates a new chat session
    pub async fn create_session(&self, title: String) -> AppResult<ChatSession> {
        let session = ChatSession {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            messages: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let mut sessions = self.sessions.write().await;
        (*sessions).push(session.clone());
        Ok(session)
    }

    /// Gets a chat session by ID
    pub async fn get_session(&self, id: &str) -> AppResult<Option<ChatSession>> {
        let sessions = self.sessions.read().await;
        Ok((*sessions).iter().find(|s| s.id == id).cloned())
    }

    /// Lists all chat sessions
    pub async fn list_sessions(&self) -> AppResult<Vec<ChatSession>> {
        let sessions = self.sessions.read().await;
        Ok((*sessions).clone())
    }

    /// Adds a message to a chat session
    pub async fn add_message(&self, session_id: &str, message: Message) -> AppResult<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(session) = (*sessions).iter_mut().find(|s| s.id == session_id) {
            session.messages.push(message);
            session.updated_at = Utc::now();
            Ok(())
        } else {
            Err(crate::utils::AppError::not_found("Chat session not found"))
        }
    }

    /// Deletes a chat session
    pub async fn delete_session(&self, id: &str) -> AppResult<()> {
        let mut sessions = self.sessions.write().await;
        if let Some(pos) = (*sessions).iter().position(|s| s.id == id) {
            (*sessions).remove(pos);
            Ok(())
        } else {
            Err(crate::utils::AppError::not_found("Chat session not found"))
        }
    }
}

impl Default for ChatManager {
    fn default() -> Self {
        Self::new()
    }
} 