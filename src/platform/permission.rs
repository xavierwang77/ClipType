use std::process::Command;

use crate::{
    error::{ClipTypeError, Result},
    services::permission::PermissionInfo,
};

#[cfg(target_os = "macos")]
pub fn info() -> PermissionInfo {
    PermissionInfo {
        accessibility_required: true,
        accessibility_granted: macos::is_accessibility_trusted(),
        can_prompt: true,
    }
}

#[cfg(target_os = "macos")]
pub fn request() -> Result<PermissionInfo> {
    let _ = macos::request_accessibility_trust();
    Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .spawn()
        .map_err(|error| {
            ClipTypeError::Platform(format!("failed to open system settings: {error}"))
        })?;
    Ok(info())
}

#[cfg(target_os = "windows")]
pub fn info() -> PermissionInfo {
    PermissionInfo {
        accessibility_required: false,
        accessibility_granted: true,
        can_prompt: false,
    }
}

#[cfg(target_os = "windows")]
pub fn request() -> Result<PermissionInfo> {
    Ok(info())
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn info() -> PermissionInfo {
    PermissionInfo {
        accessibility_required: false,
        accessibility_granted: false,
        can_prompt: false,
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
pub fn request() -> Result<PermissionInfo> {
    Ok(info())
}

#[cfg(target_os = "macos")]
mod macos {
    use std::os::raw::c_uchar;

    use core_foundation::{
        base::TCFType,
        boolean::CFBoolean,
        dictionary::{CFDictionary, CFDictionaryRef},
        string::{CFString, CFStringRef},
    };

    #[link(name = "ApplicationServices", kind = "framework")]
    unsafe extern "C" {
        fn AXIsProcessTrusted() -> c_uchar;
        fn AXIsProcessTrustedWithOptions(options: CFDictionaryRef) -> c_uchar;
        static kAXTrustedCheckOptionPrompt: CFStringRef;
    }

    pub fn is_accessibility_trusted() -> bool {
        unsafe { AXIsProcessTrusted() != 0 }
    }

    pub fn request_accessibility_trust() -> bool {
        unsafe {
            let prompt_key = CFString::wrap_under_get_rule(kAXTrustedCheckOptionPrompt);
            let prompt_value = CFBoolean::true_value();
            let options = CFDictionary::from_CFType_pairs(&[(
                prompt_key.as_CFType(),
                prompt_value.as_CFType(),
            )]);
            AXIsProcessTrustedWithOptions(options.as_concrete_TypeRef()) != 0
        }
    }
}
