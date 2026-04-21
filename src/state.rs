use std::sync::{Mutex, MutexGuard};

use serde::Serialize;

use crate::{
    config::{AppConfig, ConfigService},
    error::{ClipTypeError, Result},
    services::{
        clipboard::ClipboardService,
        hotkey::HotkeyService,
        input::InputSimulationService,
        permission::{PermissionInfo, PermissionService},
    },
};

#[derive(Debug, Clone, Serialize)]
pub struct ExecutionResult {
    pub ok: bool,
    pub message: String,
    pub typed_chars: usize,
    pub timestamp_unix_ms: u128,
}

impl ExecutionResult {
    #[must_use]
    pub fn success(typed_chars: usize) -> Self {
        Self {
            ok: true,
            message: "input completed".to_owned(),
            typed_chars,
            timestamp_unix_ms: now_unix_ms(),
        }
    }

    #[must_use]
    pub fn skipped(message: impl Into<String>) -> Self {
        Self {
            ok: false,
            message: message.into(),
            typed_chars: 0,
            timestamp_unix_ms: now_unix_ms(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AppStatus {
    pub config: AppConfig,
    pub permission: PermissionInfo,
    pub last_result: Option<ExecutionResult>,
    pub hotkey_registered: bool,
}

pub struct AppState {
    pub config_service: ConfigService,
    pub hotkey_service: HotkeyService,
    pub clipboard_service: ClipboardService,
    pub input_service: InputSimulationService,
    pub permission_service: PermissionService,
    config: Mutex<AppConfig>,
    last_result: Mutex<Option<ExecutionResult>>,
}

impl AppState {
    pub fn new(config_service: ConfigService, config: AppConfig) -> Self {
        Self {
            config_service,
            hotkey_service: HotkeyService::default(),
            clipboard_service: ClipboardService,
            input_service: InputSimulationService,
            permission_service: PermissionService,
            config: Mutex::new(config),
            last_result: Mutex::new(None),
        }
    }

    pub fn config(&self) -> Result<AppConfig> {
        Ok(self.config_guard()?.clone())
    }

    pub fn replace_config(&self, config: AppConfig) -> Result<()> {
        *self.config_guard()? = config;
        Ok(())
    }

    pub fn last_result(&self) -> Result<Option<ExecutionResult>> {
        Ok(self.last_result_guard()?.clone())
    }

    pub fn set_last_result(&self, result: ExecutionResult) -> Result<()> {
        *self.last_result_guard()? = Some(result);
        Ok(())
    }

    pub fn status(&self) -> Result<AppStatus> {
        Ok(AppStatus {
            config: self.config()?,
            permission: self.permission_service.info(),
            last_result: self.last_result()?,
            hotkey_registered: self.hotkey_service.is_registered()?,
        })
    }

    fn config_guard(&self) -> Result<MutexGuard<'_, AppConfig>> {
        self.config
            .lock()
            .map_err(|_| ClipTypeError::State("configuration lock was poisoned".to_owned()))
    }

    fn last_result_guard(&self) -> Result<MutexGuard<'_, Option<ExecutionResult>>> {
        self.last_result
            .lock()
            .map_err(|_| ClipTypeError::State("result lock was poisoned".to_owned()))
    }
}

fn now_unix_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_millis())
}
