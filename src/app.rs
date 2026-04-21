use std::{thread, time::Duration};

use anyhow::Context;
use tauri::{Manager, RunEvent};
use tauri_plugin_global_shortcut::ShortcutState;

use crate::{commands, config::ConfigService, orchestrator, state::AppState, tray};

const HOTKEY_RELEASE_SETTLE_DELAY: Duration = Duration::from_millis(80);

pub fn run() -> anyhow::Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, _shortcut, event| {
                    if event.state == ShortcutState::Released {
                        let app = app.clone();
                        tauri::async_runtime::spawn_blocking(move || {
                            thread::sleep(HOTKEY_RELEASE_SETTLE_DELAY);
                            let state = app.state::<AppState>();
                            let _ = orchestrator::type_clipboard(&app, &state);
                        });
                    }
                })
                .build(),
        )
        .setup(|app| {
            let config_dir = app
                .path()
                .app_config_dir()
                .context("failed to resolve app config directory")?;
            let config_service = ConfigService::new(config_dir.join("settings.toml"));
            let config = config_service
                .load_or_default()
                .context("failed to load settings")?;
            let state = AppState::new(config_service, config.clone());
            app.manage(state);

            let state = app.state::<AppState>();
            state
                .hotkey_service
                .sync(app.handle(), &config)
                .context("failed to register global shortcut")?;
            tray::setup(app).context("failed to create tray")?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_status,
            commands::save_settings,
            commands::set_enabled,
            commands::trigger_type_clipboard,
            commands::open_permissions,
        ])
        .build(tauri::generate_context!())
        .context("failed to build tauri app")?
        .run(|app, event| {
            if let RunEvent::ExitRequested { .. } = event
                && let Some(state) = app.try_state::<AppState>()
            {
                let _ = state.hotkey_service.unregister_current(app);
            }
        });

    Ok(())
}
