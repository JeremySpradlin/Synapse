export interface Settings {
    preferences: AppPreferences;
    ai_providers: AIProviderSettings;
}

export interface AppPreferences {
    window_width: number;
    window_height: number;
    theme: Theme;
    startup_behavior: StartupBehavior;
    keyboard_shortcuts: KeyboardShortcuts;
}

export interface AIProviderSettings {
    openai?: OpenAIConfig;
    anthropic?: AnthropicConfig;
}

export interface OpenAIConfig {
    model: string;
    temperature: number;
    max_tokens: number;
}

export interface AnthropicConfig {
    model: string;
    temperature: number;
    max_tokens: number;
}

export enum Theme {
    Light = 'light',
    Dark = 'dark',
    System = 'system'
}

export enum StartupBehavior {
    Normal = 'normal',
    Minimized = 'minimized',
    Hidden = 'hidden'
}

export interface KeyboardShortcuts {
    toggle_window: string;
    clear_conversation: string;
    new_conversation: string;
    custom_shortcuts: Record<string, string>;
}

export const DEFAULT_SETTINGS: Settings = {
    preferences: {
        window_width: 800,
        window_height: 600,
        theme: Theme.System,
        startup_behavior: StartupBehavior.Normal,
        keyboard_shortcuts: {
            toggle_window: 'CommandOrControl+Shift+Space',
            clear_conversation: 'CommandOrControl+L',
            new_conversation: 'CommandOrControl+N',
            custom_shortcuts: {
                settings: 'CommandOrControl+,'
            }
        }
    },
    ai_providers: {}
}; 