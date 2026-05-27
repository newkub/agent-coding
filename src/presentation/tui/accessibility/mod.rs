// Accessibility module for TUI
// Provides screen reader support and accessibility features

pub mod screen_reader;

pub use screen_reader::{ScreenReaderAnnouncer, AccessibilityContext};
