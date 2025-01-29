use std::sync::Arc;
use tokio::sync::RwLock;
use std::path::PathBuf;
use keyring::Entry;

mod error;
mod types;

pub use error::SettingsError;
pub use types::*;

#[derive(Debug)]
pub struct SettingsManager {
    settings: Arc<RwLock<Settings>>,
    file_path: PathBuf,
}

impl SettingsManager {
    pub async fn new() -> Result<Self, SettingsError> {
        let config_dir = tauri::api::path::config_dir()
            .ok_or(SettingsError::ConfigDirNotFound)?;
        let file_path = config_dir.join("synapse").join("settings.json");

        // Ensure the directory exists
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let settings = if file_path.exists() {
            let content = tokio::fs::read_to_string(&file_path).await?;
            serde_json::from_str(&content)?
        } else {
            Settings::default()
        };

        Ok(Self {
            settings: Arc::new(RwLock::new(settings)),
            file_path,
        })
    }

    pub async fn save(&self) -> Result<(), SettingsError> {
        let settings = self.settings.read().await;
        let content = serde_json::to_string_pretty(&*settings)?;
        
        // Write to a temporary file first
        let temp_path = self.file_path.with_extension("tmp");
        tokio::fs::write(&temp_path, content).await?;
        
        // Atomically rename the temporary file
        tokio::fs::rename(temp_path, &self.file_path).await?;
        Ok(())
    }

    pub async fn get_settings(&self) -> Result<Settings, SettingsError> {
        Ok(self.settings.read().await.clone())
    }

    pub async fn update_settings(&self, new_settings: Settings) -> Result<(), SettingsError> {
        *self.settings.write().await = new_settings;
        self.save().await?;
        Ok(())
    }

    pub async fn store_api_key(&self, provider: &str, key: &str) -> Result<(), SettingsError> {
        let keyring = Entry::new("synapse", provider)?;
        keyring.set_password(key)?;
        Ok(())
    }

    pub async fn get_api_key(&self, provider: &str) -> Result<String, SettingsError> {
        let keyring = Entry::new("synapse", provider)?;
        Ok(keyring.get_password()?)
    }

    pub async fn delete_api_key(&self, provider: &str) -> Result<(), SettingsError> {
        let keyring = Entry::new("synapse", provider)?;
        keyring.delete_password()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_settings_crud() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("settings.json");
        
        let manager = SettingsManager::new().await.unwrap();
        
        // Test default settings
        let settings = manager.get_settings().await.unwrap();
        assert_eq!(settings.preferences.window_width, 800); // Assuming default value
        
        // Test updating settings
        let mut new_settings = settings.clone();
        new_settings.preferences.window_width = 1000;
        manager.update_settings(new_settings.clone()).await.unwrap();
        
        // Verify update
        let updated_settings = manager.get_settings().await.unwrap();
        assert_eq!(updated_settings.preferences.window_width, 1000);
    }
} 