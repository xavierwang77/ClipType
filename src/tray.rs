use tauri::{
    App, AppHandle, Manager,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

use crate::{
    commands,
    error::{ClipTypeError, Result},
    state::AppState,
};

pub fn setup(app: &mut App) -> Result<()> {
    let show = MenuItem::with_id(app, "show", "Open Settings", true, None::<&str>)?;
    let toggle = MenuItem::with_id(app, "toggle", "Enable / Disable", true, None::<&str>)?;
    let permissions =
        MenuItem::with_id(app, "permissions", "Permission Status", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let menu = Menu::with_items(app, &[&show, &toggle, &permissions, &separator, &quit])?;

    let mut tray_builder = TrayIconBuilder::with_id("main")
        .tooltip("ClipType")
        .menu(&menu)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => {
                let _ = show_main_window(app);
            }
            "toggle" => {
                let _ = toggle_enabled(app);
            }
            "permissions" => {
                let _ = refresh_permissions(app);
            }
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let _ = show_main_window(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon().cloned() {
        tray_builder = tray_builder.icon(icon);
    }

    tray_builder.build(app).map_err(ClipTypeError::Tauri)?;

    Ok(())
}

fn show_main_window(app: &AppHandle) -> Result<()> {
    let window = app
        .get_webview_window("main")
        .ok_or_else(|| ClipTypeError::State("main window is not available".to_owned()))?;
    window.show()?;
    window.set_focus()?;
    Ok(())
}

fn toggle_enabled(app: &AppHandle) -> Result<()> {
    let state = app.state::<AppState>();
    let enabled = !state.config()?.enabled;
    let _ = commands::set_enabled(app.clone(), state, enabled).map_err(ClipTypeError::State)?;
    Ok(())
}

fn refresh_permissions(app: &AppHandle) -> Result<()> {
    let state = app.state::<AppState>();
    let _ = commands::open_permissions(app.clone(), state).map_err(ClipTypeError::State)?;
    Ok(())
}
