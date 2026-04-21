use crate::{config::TypingSettings, error::Result, platform};

pub struct InputSimulationService;

impl InputSimulationService {
    pub fn type_text(&self, text: &str, settings: TypingSettings) -> Result<()> {
        platform::input::type_text(text, settings.delay, settings.append_enter)
    }
}
