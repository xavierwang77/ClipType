use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

use crate::error::{ClipTypeError, Result};

pub struct ClipboardService;

impl ClipboardService {
    pub fn read_text(&self, app: &AppHandle) -> Result<String> {
        app.clipboard()
            .read_text()
            .map_err(|error| ClipTypeError::Clipboard(error.to_string()))
    }
}
