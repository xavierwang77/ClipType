use std::{thread, time::Duration};

use crate::error::{ClipTypeError, Result};

#[cfg(target_os = "macos")]
pub fn type_text(text: &str, delay: Duration, append_enter: bool) -> Result<()> {
    macos::type_text(text, delay, append_enter)
}

#[cfg(target_os = "windows")]
pub fn type_text(text: &str, delay: Duration, append_enter: bool) -> Result<()> {
    windows::type_text(text, delay, append_enter)
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn type_text(_text: &str, _delay: Duration, _append_enter: bool) -> Result<()> {
    Err(ClipTypeError::Platform(
        "input simulation is currently implemented for macOS and Windows".to_owned(),
    ))
}

fn pause(delay: Duration) {
    if !delay.is_zero() {
        thread::sleep(delay);
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use core_graphics::{
        event::{CGEvent, CGEventFlags, CGEventTapLocation},
        event_source::{CGEventSource, CGEventSourceStateID},
    };

    use super::{ClipTypeError, Duration, Result, pause};
    use crate::platform::permission;

    const RETURN_KEY_CODE: u16 = 36;

    pub fn type_text(text: &str, delay: Duration, append_enter: bool) -> Result<()> {
        let permission = permission::info();
        if permission.accessibility_required && !permission.accessibility_granted {
            return Err(ClipTypeError::Permission(
                "accessibility permission is required before typing".to_owned(),
            ));
        }

        let source = CGEventSource::new(CGEventSourceStateID::Private).map_err(|error| {
            ClipTypeError::Input(format!("failed to create event source: {error:?}"))
        })?;

        for ch in text.chars() {
            post_unicode(&source, ch)?;
            pause(delay);
        }

        if append_enter {
            post_key(&source, RETURN_KEY_CODE)?;
        }

        Ok(())
    }

    fn post_unicode(source: &CGEventSource, ch: char) -> Result<()> {
        let text = ch.to_string();
        let key_down = CGEvent::new_keyboard_event(source.clone(), 0, true).map_err(|error| {
            ClipTypeError::Input(format!("failed to create key down event: {error:?}"))
        })?;
        key_down.set_flags(CGEventFlags::empty());
        key_down.set_string(&text);
        key_down.post(CGEventTapLocation::HID);

        let key_up = CGEvent::new_keyboard_event(source.clone(), 0, false).map_err(|error| {
            ClipTypeError::Input(format!("failed to create key up event: {error:?}"))
        })?;
        key_up.set_flags(CGEventFlags::empty());
        key_up.post(CGEventTapLocation::HID);
        Ok(())
    }

    fn post_key(source: &CGEventSource, key_code: u16) -> Result<()> {
        let key_down =
            CGEvent::new_keyboard_event(source.clone(), key_code, true).map_err(|error| {
                ClipTypeError::Input(format!("failed to create key down event: {error:?}"))
            })?;
        key_down.set_flags(CGEventFlags::empty());
        key_down.post(CGEventTapLocation::HID);

        let key_up =
            CGEvent::new_keyboard_event(source.clone(), key_code, false).map_err(|error| {
                ClipTypeError::Input(format!("failed to create key up event: {error:?}"))
            })?;
        key_up.set_flags(CGEventFlags::empty());
        key_up.post(CGEventTapLocation::HID);
        Ok(())
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        INPUT, INPUT_0, INPUT_KEYBOARD, KEYBD_EVENT_FLAGS, KEYBDINPUT, KEYEVENTF_KEYUP,
        KEYEVENTF_UNICODE, SendInput, VIRTUAL_KEY, VK_RETURN,
    };

    use super::{ClipTypeError, Duration, Result, pause};

    pub fn type_text(text: &str, delay: Duration, append_enter: bool) -> Result<()> {
        for code_unit in text.encode_utf16() {
            send_unicode(code_unit)?;
            pause(delay);
        }

        if append_enter {
            send_virtual_key(VK_RETURN.0)?;
        }

        Ok(())
    }

    fn send_unicode(code_unit: u16) -> Result<()> {
        send_pair(
            keyboard_input(VIRTUAL_KEY(0), code_unit, KEYEVENTF_UNICODE),
            keyboard_input(
                VIRTUAL_KEY(0),
                code_unit,
                KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
            ),
        )
    }

    fn send_virtual_key(key: u16) -> Result<()> {
        send_pair(
            keyboard_input(VIRTUAL_KEY(key), 0, KEYBD_EVENT_FLAGS(0)),
            keyboard_input(VIRTUAL_KEY(key), 0, KEYEVENTF_KEYUP),
        )
    }

    fn send_pair(key_down: INPUT, key_up: INPUT) -> Result<()> {
        let inputs = [key_down, key_up];
        let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
        if sent == inputs.len() as u32 {
            Ok(())
        } else {
            Err(ClipTypeError::Input(format!(
                "SendInput sent {sent} of {} input events",
                inputs.len()
            )))
        }
    }

    fn keyboard_input(w_vk: VIRTUAL_KEY, w_scan: u16, flags: KEYBD_EVENT_FLAGS) -> INPUT {
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: w_vk,
                    wScan: w_scan,
                    dwFlags: flags,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        }
    }
}
