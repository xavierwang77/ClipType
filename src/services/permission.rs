use serde::Serialize;

use crate::platform;

#[derive(Debug, Clone, Serialize)]
pub struct PermissionInfo {
    pub accessibility_required: bool,
    pub accessibility_granted: bool,
    pub can_prompt: bool,
}

pub struct PermissionService;

impl PermissionService {
    #[must_use]
    pub fn info(&self) -> PermissionInfo {
        platform::permission::info()
    }

    pub fn request(&self) -> crate::error::Result<PermissionInfo> {
        platform::permission::request()
    }
}
