import { invoke } from '@tauri-apps/api/tauri';
import type { Settings, Theme, StartupBehavior } from '../../types/settings';

class SettingsManager {
  private settings: Settings | null = null;
  private hasUnsavedChanges = false;
  private autoSaveTimeout: number | null = null;
  private readonly AUTO_SAVE_DELAY = 2000; // 2 seconds

  private elements = {
    openaiKey: document.getElementById('openai-api-key') as HTMLInputElement,
    openaiModel: document.getElementById('openai-model') as HTMLSelectElement,
    anthropicKey: document.getElementById('anthropic-api-key') as HTMLInputElement,
    anthropicModel: document.getElementById('anthropic-model') as HTMLSelectElement,
    theme: document.getElementById('theme') as HTMLSelectElement,
    startMinimized: document.getElementById('start-minimized') as HTMLInputElement,
    saveButton: document.getElementById('save-settings') as HTMLButtonElement,
    saveStatus: document.getElementById('save-status') as HTMLSpanElement,
  };

  constructor() {
    this.initializeSettings();
    this.setupEventListeners();
  }

  private async initializeSettings() {
    try {
      this.updateSaveStatus('loading', 'Loading settings...');
      this.settings = await invoke<Settings>('get_settings');
      this.updateUIFromSettings();
      this.updateSaveStatus('saved', 'All changes saved');
    } catch (err) {
      console.error('Failed to load settings:', err);
      this.updateSaveStatus('error', 'Failed to load settings');
    }
  }

  private updateUIFromSettings() {
    if (!this.settings) return;

    // Update OpenAI settings
    if (this.settings.ai_providers.openai) {
      this.elements.openaiModel.value = this.settings.ai_providers.openai.model;
    }

    // Update Anthropic settings
    if (this.settings.ai_providers.anthropic) {
      this.elements.anthropicModel.value = this.settings.ai_providers.anthropic.model;
    }

    // Update appearance settings
    this.elements.theme.value = this.settings.preferences.theme;

    // Update behavior settings
    this.elements.startMinimized.checked = 
      this.settings.preferences.startup_behavior === 'minimized';
  }

  private setupEventListeners() {
    // Save button handling
    this.elements.saveButton.addEventListener('click', () => {
      void this.saveChanges();
    });

    // API Key handling
    this.elements.openaiKey.addEventListener('change', (e) => {
      const target = e.target as HTMLInputElement;
      this.markUnsaved();
      void this.handleAPIKeyUpdate('openai', target.value);
    });

    this.elements.anthropicKey.addEventListener('change', (e) => {
      const target = e.target as HTMLInputElement;
      this.markUnsaved();
      void this.handleAPIKeyUpdate('anthropic', target.value);
    });

    // Model selection handling
    this.elements.openaiModel.addEventListener('change', (e) => {
      if (!this.settings?.ai_providers.openai) return;
      
      const target = e.target as HTMLSelectElement;
      this.markUnsaved();
      this.queueAutoSave({
        ...this.settings,
        ai_providers: {
          ...this.settings.ai_providers,
          openai: {
            ...this.settings.ai_providers.openai,
            model: target.value
          }
        }
      });
    });

    this.elements.anthropicModel.addEventListener('change', (e) => {
      if (!this.settings?.ai_providers.anthropic) return;
      
      const target = e.target as HTMLSelectElement;
      this.markUnsaved();
      this.queueAutoSave({
        ...this.settings,
        ai_providers: {
          ...this.settings.ai_providers,
          anthropic: {
            ...this.settings.ai_providers.anthropic,
            model: target.value
          }
        }
      });
    });

    // Theme handling
    this.elements.theme.addEventListener('change', (e) => {
      if (!this.settings) return;
      
      const target = e.target as HTMLSelectElement;
      const value = target.value as Theme;
      this.markUnsaved();
      this.queueAutoSave({
        ...this.settings,
        preferences: {
          ...this.settings.preferences,
          theme: value
        }
      });
    });

    // Start minimized handling
    this.elements.startMinimized.addEventListener('change', (e) => {
      if (!this.settings) return;
      
      const target = e.target as HTMLInputElement;
      const value: StartupBehavior = target.checked ? 'minimized' : 'normal';
      this.markUnsaved();
      this.queueAutoSave({
        ...this.settings,
        preferences: {
          ...this.settings.preferences,
          startup_behavior: value
        }
      });
    });

    // Handle unsaved changes when closing
    window.addEventListener('beforeunload', (e) => {
      if (this.hasUnsavedChanges) {
        e.preventDefault();
        e.returnValue = '';
      }
    });
  }

  private markUnsaved() {
    this.hasUnsavedChanges = true;
    this.elements.saveButton.disabled = false;
    this.updateSaveStatus('unsaved', 'You have unsaved changes');
  }

  private updateSaveStatus(
    status: 'loading' | 'saved' | 'unsaved' | 'saving' | 'error',
    message: string
  ) {
    this.elements.saveStatus.textContent = message;
    this.elements.saveStatus.className = `save-status status-${status}`;
    
    if (status === 'saved') {
      this.hasUnsavedChanges = false;
      this.elements.saveButton.disabled = true;
    } else if (status === 'error') {
      this.elements.saveButton.disabled = false;
    }
  }

  private queueAutoSave(newSettings: Settings) {
    if (this.autoSaveTimeout) {
      window.clearTimeout(this.autoSaveTimeout);
    }

    this.autoSaveTimeout = window.setTimeout(() => {
      void this.saveChanges(newSettings);
    }, this.AUTO_SAVE_DELAY);
  }

  private async handleAPIKeyUpdate(provider: string, key: string) {
    try {
      this.updateSaveStatus('saving', 'Saving API key...');
      await invoke('store_api_key', { provider, key });
      this.updateSaveStatus('saved', 'API key saved');
    } catch (err) {
      console.error(`Failed to store ${provider} API key:`, err);
      this.updateSaveStatus('error', `Failed to save ${provider} API key`);
    }
  }

  private async saveChanges(newSettings?: Settings) {
    try {
      this.updateSaveStatus('saving', 'Saving changes...');
      
      if (newSettings) {
        await invoke('update_settings', { settings: newSettings });
        this.settings = newSettings;
      } else if (this.settings) {
        await invoke('update_settings', { settings: this.settings });
      }

      this.updateSaveStatus('saved', 'All changes saved');
    } catch (err) {
      console.error('Failed to update settings:', err);
      this.updateSaveStatus('error', 'Failed to save changes');
    }
  }
}

// Initialize settings manager when the DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
  new SettingsManager();
}); 
