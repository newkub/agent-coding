#![allow(dead_code)]
use crate::adapters::config::settings::AccessibilitySettings;

/// Screen reader announcer for accessibility
pub(crate) struct ScreenReaderAnnouncer {
    enabled: bool,
    announce_focus: bool,
    announce_content: bool,
}

impl ScreenReaderAnnouncer {
    pub(crate) const fn new(settings: &AccessibilitySettings) -> Self {
        Self {
            enabled: settings.enable_screen_reader,
            announce_focus: settings.announce_focus,
            announce_content: settings.announce_content,
        }
    }

    pub(crate) const fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub(crate) fn announce(&self, message: &str) {
        if !self.enabled {
            return;
        }

        // In a real implementation, this would use a screen reader API
        // For now, we'll output to stderr which can be captured by screen readers
        eprintln!("[SCREEN READER] {}", message);
    }

    pub(crate) fn announce_focus(&self, element: &str, context: Option<&str>) {
        if !self.enabled || !self.announce_focus {
            return;
        }

        let message = if let Some(ctx) = context {
            format!("Focused on {}: {}", element, ctx)
        } else {
            format!("Focused on {}", element)
        };

        self.announce(&message);
    }

    pub(crate) fn announce_content(&self, content: &str) {
        if !self.enabled || !self.announce_content {
            return;
        }

        self.announce(&format!("Content: {}", content));
    }

    pub(crate) fn announce_action(&self, action: &str, target: Option<&str>) {
        if !self.enabled {
            return;
        }

        let message = if let Some(t) = target {
            format!("{}: {}", action, t)
        } else {
            action.to_string()
        };

        self.announce(&message);
    }

    pub(crate) fn announce_error(&self, error: &str) {
        if !self.enabled {
            return;
        }

        self.announce(&format!("Error: {}", error));
    }

    pub(crate) fn announce_success(&self, message: &str) {
        if !self.enabled {
            return;
        }

        self.announce(&format!("Success: {}", message));
    }

    pub(crate) fn update_settings(&mut self, settings: &AccessibilitySettings) {
        self.enabled = settings.enable_screen_reader;
        self.announce_focus = settings.announce_focus;
        self.announce_content = settings.announce_content;
    }
}

impl Default for ScreenReaderAnnouncer {
    fn default() -> Self {
        Self {
            enabled: false,
            announce_focus: true,
            announce_content: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_announcer_disabled() {
        let settings = AccessibilitySettings {
            enable_screen_reader: false,
            high_contrast: false,
            reduced_motion: false,
            announce_focus: true,
            announce_content: true,
            theme: "default".to_string(),
            font_scale: 1.0,
        };
        let announcer = ScreenReaderAnnouncer::new(&settings);

        assert!(!announcer.is_enabled());
    }

    #[test]
    fn test_announcer_enabled() {
        let settings = AccessibilitySettings {
            enable_screen_reader: true,
            high_contrast: false,
            reduced_motion: false,
            announce_focus: true,
            announce_content: true,
            theme: "default".to_string(),
            font_scale: 1.0,
        };
        let announcer = ScreenReaderAnnouncer::new(&settings);

        assert!(announcer.is_enabled());
    }

    #[test]
    fn test_update_settings() {
        let mut announcer = ScreenReaderAnnouncer::default();

        let settings = AccessibilitySettings {
            enable_screen_reader: true,
            high_contrast: false,
            reduced_motion: false,
            announce_focus: false,
            announce_content: false,
            theme: "default".to_string(),
            font_scale: 1.0,
        };

        announcer.update_settings(&settings);
        assert!(announcer.is_enabled());
    }
}
