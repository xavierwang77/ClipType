use tauri::{AppHandle, Emitter};

use crate::{
    error::{ClipTypeError, Result},
    state::{AppState, ExecutionResult},
};

pub fn type_clipboard(app: &AppHandle, state: &AppState) -> Result<ExecutionResult> {
    let result = match execute_type_clipboard(app, state) {
        Ok(result) => result,
        Err(error) => ExecutionResult::skipped(error.to_string()),
    };

    state.set_last_result(result.clone())?;
    let _ = app.emit("cliptype-result", &result);
    Ok(result)
}

fn execute_type_clipboard(app: &AppHandle, state: &AppState) -> Result<ExecutionResult> {
    let config = state.config()?;
    if !config.enabled {
        return Ok(ExecutionResult::skipped("ClipType is disabled"));
    }

    let permission = state.permission_service.info();
    if permission.accessibility_required && !permission.accessibility_granted {
        return Err(ClipTypeError::Permission(
            "accessibility permission is required before typing".to_owned(),
        ));
    }

    let text = state.clipboard_service.read_text(app)?;
    if text.is_empty() {
        return Ok(ExecutionResult::skipped("clipboard text is empty"));
    }

    let typed_chars = text.chars().count();
    state
        .input_service
        .type_text(&text, config.typing_settings())?;

    Ok(ExecutionResult::success(typed_chars))
}
