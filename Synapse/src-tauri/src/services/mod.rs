//! Services module
//! 
//! This module contains core application services:
//! - AI providers and chat completion
//! - Chat session management

pub mod ai;
pub mod chat;

pub use chat::ChatManager; 