use std::{fs, path::PathBuf, time::Duration};

use serde::{Deserialize, Serialize};

use crate::error::Result;

pub const DEFAULT_HOTKEY: &str = "CommandOrControl+Shift+V";
pub const DEFAULT_INPUT_DELAY_MS: u64 = 8;
pub const MAX_INPUT_DELAY_MS: u64 = 1000;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct AppConfig {
    pub enabled: bool,
    pub hotkey: String,
    pub input_delay_ms: u64,
    pub append_enter: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            hotkey: DEFAULT_HOTKEY.to_owned(),
            input_delay_ms: DEFAULT_INPUT_DELAY_MS,
            append_enter: false,
        }
    }
}

impl AppConfig {
    #[must_use]
    pub fn normalized(mut self) -> Self {
        self.hotkey = self.hotkey.trim().to_owned();
        if self.hotkey.is_empty() {
            self.hotkey = DEFAULT_HOTKEY.to_owned();
        }
        self.input_delay_ms = self.input_delay_ms.min(MAX_INPUT_DELAY_MS);
        self
    }

    #[must_use]
    pub fn typing_settings(&self) -> TypingSettings {
        TypingSettings {
            delay: Duration::from_millis(self.input_delay_ms),
            append_enter: self.append_enter,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypingSettings {
    pub delay: Duration,
    pub append_enter: bool,
}

#[derive(Debug, Clone)]
pub struct ConfigService {
    path: PathBuf,
}

impl ConfigService {
    #[must_use]
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn load_or_default(&self) -> Result<AppConfig> {
        if !self.path.try_exists()? {
            return Ok(AppConfig::default());
        }

        let content = fs::read_to_string(&self.path)?;
        let config = toml::from_str::<AppConfig>(&content)?;
        Ok(config.normalized())
    }

    pub fn save(&self, config: &AppConfig) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(&config.clone().normalized())?;
        fs::write(&self.path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_empty_hotkey_and_clamps_delay() {
        let config = AppConfig {
            enabled: true,
            hotkey: "   ".to_owned(),
            input_delay_ms: 2000,
            append_enter: true,
        }
        .normalized();

        assert_eq!(config.hotkey, DEFAULT_HOTKEY);
        assert_eq!(config.input_delay_ms, MAX_INPUT_DELAY_MS);
        assert!(config.append_enter);
    }

    #[test]
    fn default_typing_settings_use_millisecond_delay() {
        let settings = AppConfig::default().typing_settings();

        assert_eq!(
            settings.delay,
            Duration::from_millis(DEFAULT_INPUT_DELAY_MS)
        );
        assert!(!settings.append_enter);
    }
}
