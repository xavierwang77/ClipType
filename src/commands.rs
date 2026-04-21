use tauri::{AppHandle, Emitter, State};

use crate::{
    config::AppConfig,
    error::Result,
    orchestrator,
    state::{AppState, AppStatus, ExecutionResult},
};

#[tauri::command]
pub fn get_status(state: State<'_, AppState>) -> std::result::Result<AppStatus, String> {
    state.status().map_err(format_error)
}

#[tauri::command]
pub fn save_settings(
    app: AppHandle,
    state: State<'_, AppState>,
    config: AppConfig,
) -> std::result::Result<AppStatus, String> {
    save_config_and_refresh_hotkey(&app, &state, config).map_err(format_error)
}

#[tauri::command]
pub fn set_enabled(
    app: AppHandle,
    state: State<'_, AppState>,
    enabled: bool,
) -> std::result::Result<AppStatus, String> {
    let mut config = state.config().map_err(format_error)?;
    config.enabled = enabled;
    save_config_and_refresh_hotkey(&app, &state, config).map_err(format_error)
}

#[tauri::command]
pub fn trigger_type_clipboard(
    app: AppHandle,
    state: State<'_, AppState>,
) -> std::result::Result<ExecutionResult, String> {
    orchestrator::type_clipboard(&app, &state).map_err(format_error)
}

#[tauri::command]
pub fn open_permissions(
    app: AppHandle,
    state: State<'_, AppState>,
) -> std::result::Result<AppStatus, String> {
    let _ = state.permission_service.request().map_err(format_error)?;
    let status = state.status().map_err(format_error)?;
    let _ = app.emit("cliptype-status", &status);
    Ok(status)
}

fn save_config_and_refresh_hotkey(
    app: &AppHandle,
    state: &AppState,
    config: AppConfig,
) -> Result<AppStatus> {
    let config = config.normalized();
    state.config_service.save(&config)?;
    state.hotkey_service.sync(app, &config)?;
    state.replace_config(config)?;

    let status = state.status()?;
    let _ = app.emit("cliptype-status", &status);
    Ok(status)
}

fn format_error(error: crate::error::ClipTypeError) -> String {
    error.to_string()
}
