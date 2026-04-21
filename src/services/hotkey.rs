use std::sync::{Mutex, MutexGuard};

use tauri::AppHandle;
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

use crate::{
    config::AppConfig,
    error::{ClipTypeError, Result},
};

#[derive(Default)]
pub struct HotkeyService {
    registered: Mutex<Option<String>>,
}

impl HotkeyService {
    pub fn sync(&self, app: &AppHandle, config: &AppConfig) -> Result<()> {
        self.unregister_current(app)?;

        if !config.enabled {
            return Ok(());
        }

        let shortcut = parse_shortcut(&config.hotkey)?;
        app.global_shortcut()
            .register(shortcut)
            .map_err(|error| ClipTypeError::Hotkey(error.to_string()))?;

        *self.registered_guard()? = Some(config.hotkey.clone());
        Ok(())
    }

    pub fn unregister_current(&self, app: &AppHandle) -> Result<()> {
        let mut registered = self.registered_guard()?;
        if let Some(hotkey) = registered.take() {
            let shortcut = parse_shortcut(&hotkey)?;
            app.global_shortcut()
                .unregister(shortcut)
                .map_err(|error| ClipTypeError::Hotkey(error.to_string()))?;
        }
        Ok(())
    }

    pub fn is_registered(&self) -> Result<bool> {
        Ok(self.registered_guard()?.is_some())
    }

    fn registered_guard(&self) -> Result<MutexGuard<'_, Option<String>>> {
        self.registered
            .lock()
            .map_err(|_| ClipTypeError::State("hotkey lock was poisoned".to_owned()))
    }
}

fn parse_shortcut(hotkey: &str) -> Result<Shortcut> {
    hotkey
        .parse::<Shortcut>()
        .map_err(|error| ClipTypeError::Hotkey(format!("invalid shortcut `{hotkey}`: {error}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DEFAULT_HOTKEY;

    #[test]
    fn default_hotkey_is_parseable() {
        let shortcut = parse_shortcut(DEFAULT_HOTKEY);

        assert!(shortcut.is_ok());
    }
}
