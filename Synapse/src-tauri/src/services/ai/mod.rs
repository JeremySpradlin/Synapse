//! AI Service module
//! 
//! This module handles interactions with AI providers like OpenAI and Anthropic.
//! It provides a unified interface for making chat completions and managing API keys.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::fmt::Debug;
use crate::utils::AppResult;

/// Represents a chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// The role of the message sender (system, user, assistant)
    pub role: String,
    /// The content of the message
    pub content: String,
    /// Timestamp of when the message was created
    pub timestamp: i64,
}

/// Represents chat completion parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletionParams {
    /// The model to use for completion
    pub model: String,
    /// Temperature for response generation (0.0 to 1.0)
    pub temperature: f32,
    /// Maximum tokens to generate
    pub max_tokens: i32,
    /// System prompt to use
    pub system_prompt: Option<String>,
}

/// Represents a chat completion response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatCompletion {
    /// The generated message
    pub message: Message,
    /// Usage statistics
    pub usage: CompletionUsage,
}

/// Represents token usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionUsage {
    /// Number of prompt tokens used
    pub prompt_tokens: i32,
    /// Number of completion tokens used
    pub completion_tokens: i32,
    /// Total tokens used
    pub total_tokens: i32,
}

/// Trait that must be implemented by all AI providers
#[async_trait]
pub trait AIProvider: Send + Sync + Debug {
    /// Returns the name of the provider
    fn name(&self) -> &str;
    
    /// Returns the available models
    fn available_models(&self) -> Vec<String>;
    
    /// Creates a chat completion
    async fn create_chat_completion(
        &self,
        messages: Vec<Message>,
        params: ChatCompletionParams
    ) -> AppResult<ChatCompletion>;
    
    /// Validates the API key
    async fn validate_api_key(&self, api_key: &str) -> AppResult<bool>;
}

/// Factory for creating AI providers
pub struct AIProviderFactory;

impl AIProviderFactory {
    /// Creates a new AI provider instance based on the provider name
    pub async fn create_provider(
        provider_name: &str,
        _api_key: String,
    ) -> AppResult<Arc<dyn AIProvider>> {
        match provider_name {
            // We'll implement these providers later
            "openai" => Err(crate::utils::AppError::invalid_input("OpenAI provider not implemented yet")),
            "anthropic" => Err(crate::utils::AppError::invalid_input("Anthropic provider not implemented yet")),
            _ => Err(crate::utils::AppError::invalid_input("Unknown provider")),
        }
    }
} 