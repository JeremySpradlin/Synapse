use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub preferences: AppPreferences,
    pub ai_providers: AIProviderSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            preferences: AppPreferences::default(),
            ai_providers: AIProviderSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppPreferences {
    pub window_width: u32,
    pub window_height: u32,
    pub theme: Theme,
    pub startup_behavior: StartupBehavior,
    pub keyboard_shortcuts: KeyboardShortcuts,
}

impl Default for AppPreferences {
    fn default() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
            theme: Theme::System,
            startup_behavior: StartupBehavior::Normal,
            keyboard_shortcuts: KeyboardShortcuts::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderSettings {
    pub openai: Option<OpenAIConfig>,
    pub anthropic: Option<AnthropicConfig>,
}

impl Default for AIProviderSettings {
    fn default() -> Self {
        Self {
            openai: None,
            anthropic: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAIConfig {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnthropicConfig {
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StartupBehavior {
    Normal,
    Minimized,
    Hidden,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardShortcuts {
    pub toggle_window: String,
    pub clear_conversation: String,
    pub new_conversation: String,
    pub custom_shortcuts: HashMap<String, String>,
}

impl Default for KeyboardShortcuts {
    fn default() -> Self {
        let mut custom_shortcuts = HashMap::new();
        custom_shortcuts.insert("settings".to_string(), "CommandOrControl+,".to_string());
        
        Self {
            toggle_window: "CommandOrControl+Shift+Space".to_string(),
            clear_conversation: "CommandOrControl+L".to_string(),
            new_conversation: "CommandOrControl+N".to_string(),
            custom_shortcuts,
        }
    }
}

// Validation traits
pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}

impl Validate for Settings {
    fn validate(&self) -> Result<(), String> {
        self.preferences.validate()?;
        self.ai_providers.validate()?;
        Ok(())
    }
}

impl Validate for AppPreferences {
    fn validate(&self) -> Result<(), String> {
        if self.window_width < 400 {
            return Err("Window width must be at least 400 pixels".to_string());
        }
        if self.window_height < 300 {
            return Err("Window height must be at least 300 pixels".to_string());
        }
        self.keyboard_shortcuts.validate()?;
        Ok(())
    }
}

impl Validate for AIProviderSettings {
    fn validate(&self) -> Result<(), String> {
        if let Some(openai) = &self.openai {
            if openai.temperature < 0.0 || openai.temperature > 1.0 {
                return Err("OpenAI temperature must be between 0 and 1".to_string());
            }
        }
        if let Some(anthropic) = &self.anthropic {
            if anthropic.temperature < 0.0 || anthropic.temperature > 1.0 {
                return Err("Anthropic temperature must be between 0 and 1".to_string());
            }
        }
        Ok(())
    }
}

impl Validate for KeyboardShortcuts {
    fn validate(&self) -> Result<(), String> {
        let validate_shortcut = |shortcut: &str| -> Result<(), String> {
            if !shortcut.contains("CommandOrControl") && !shortcut.contains("Alt") {
                return Err(format!("Invalid shortcut format: {}", shortcut));
            }
            Ok(())
        };

        validate_shortcut(&self.toggle_window)?;
        validate_shortcut(&self.clear_conversation)?;
        validate_shortcut(&self.new_conversation)?;

        for (_, shortcut) in &self.custom_shortcuts {
            validate_shortcut(shortcut)?;
        }

        Ok(())
    }
} 