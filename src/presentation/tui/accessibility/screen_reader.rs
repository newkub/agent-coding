// Screen reader support for TUI
// Provides announcements and accessibility features for screen readers

use crate::adapters::config::settings::AccessibilitySettings;
use std::sync::{Arc, Mutex};

/// Screen reader announcer
pub struct ScreenReaderAnnouncer {
    enabled: bool,
    buffer: Arc<Mutex<Vec<String>>>,
    announce_focus: bool,
    announce_content: bool,
}

impl ScreenReaderAnnouncer {
    /// Create new screen reader announcer
    pub(crate) fn new(settings: &AccessibilitySettings) -> Self {
        Self {
            enabled: settings.enable_screen_reader,
            buffer: Arc::new(Mutex::new(Vec::new())),
            announce_focus: settings.announce_focus,
            announce_content: settings.announce_content,
        }
    }

    /// Announce a message
    pub(crate) fn announce(&self, message: &str) {
        if !self.enabled {
            return;
        }

        let mut buffer = self.buffer.lock().unwrap();
        buffer.push(message.to_string());

        // In a real implementation, this would:
        // 1. Write to a named pipe or socket that screen readers monitor
        // 2. Use platform-specific APIs (e.g., Windows SAPI, macOS VoiceOver)
        // 3. Emit ANSI escape codes for terminal screen readers

        // For now, we'll just print to stderr (which some screen readers monitor)
        eprintln!("[SCREEN READER] {}", message);
    }

    /// Announce focus change
    pub(crate) fn announce_focus(&self, element: &str) {
        if self.announce_focus {
            self.announce(&format!("Focused on {}", element));
        }
    }

    /// Announce content change
    pub(crate) fn announce_content_change(&self, element: &str, content: &str) {
        if self.announce_content {
            self.announce(&format!("{} changed to: {}", element, content));
        }
    }

    /// Announce error
    pub(crate) fn announce_error(&self, error: &str) {
        self.announce(&format!("Error: {}", error));
    }

    /// Announce success
    pub(crate) fn announce_success(&self, message: &str) {
        self.announce(&format!("Success: {}", message));
    }

    /// Get announcement buffer (for testing)
    pub(crate) fn get_announcements(&self) -> Vec<String> {
        let buffer = self.buffer.lock().unwrap();
        buffer.clone()
    }

    /// Clear announcement buffer
    pub(crate) fn clear_announcements(&self) {
        let mut buffer = self.buffer.lock().unwrap();
        buffer.clear();
    }

    /// Check if screen reader is enabled
    pub(crate) const fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Accessibility context for TUI
pub struct AccessibilityContext {
    announcer: ScreenReaderAnnouncer,
    high_contrast: bool,
    reduced_motion: bool,
    font_scale: f32,
}

impl AccessibilityContext {
    /// Create new accessibility context
    pub(crate) fn new(settings: &AccessibilitySettings) -> Self {
        Self {
            announcer: ScreenReaderAnnouncer::new(settings),
            high_contrast: settings.high_contrast,
            reduced_motion: settings.reduced_motion,
            font_scale: settings.font_scale,
        }
    }

    /// Get screen reader announcer
    pub(crate) const fn announcer(&self) -> &ScreenReaderAnnouncer {
        &self.announcer
    }

    /// Check if high contrast mode is enabled
    pub(crate) const fn is_high_contrast(&self) -> bool {
        self.high_contrast
    }

    /// Check if reduced motion is enabled
    pub(crate) const fn is_reduced_motion(&self) -> bool {
        self.reduced_motion
    }

    /// Get font scale
    pub(crate) const fn font_scale(&self) -> f32 {
        self.font_scale
    }

    /// Update settings
    pub(crate) fn update_settings(&mut self, settings: &AccessibilitySettings) {
        self.announcer = ScreenReaderAnnouncer::new(settings);
        self.high_contrast = settings.high_contrast;
        self.reduced_motion = settings.reduced_motion;
        self.font_scale = settings.font_scale;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_announcer_disabled() {
        let settings = AccessibilitySettings {
            enable_screen_reader: false,
            ..Default::default()
        };
        let announcer = ScreenReaderAnnouncer::new(&settings);
        announcer.announce("Test message");
        assert!(announcer.get_announcements().is_empty());
    }

    #[test]
    fn test_announcer_enabled() {
        let settings = AccessibilitySettings {
            enable_screen_reader: true,
            ..Default::default()
        };
        let announcer = ScreenReaderAnnouncer::new(&settings);
        announcer.announce("Test message");
        assert_eq!(announcer.get_announcements().len(), 1);
    }

    #[test]
    fn test_focus_announcement() {
        let settings = AccessibilitySettings {
            enable_screen_reader: true,
            announce_focus: true,
            ..Default::default()
        };
        let announcer = ScreenReaderAnnouncer::new(&settings);
        announcer.announce_focus("Button");
        assert_eq!(announcer.get_announcements().len(), 1);
    }

    #[test]
    fn test_content_announcement() {
        let settings = AccessibilitySettings {
            enable_screen_reader: true,
            announce_content: true,
            ..Default::default()
        };
        let announcer = ScreenReaderAnnouncer::new(&settings);
        announcer.announce_content_change("Input", "Hello");
        assert_eq!(announcer.get_announcements().len(), 1);
    }
}
