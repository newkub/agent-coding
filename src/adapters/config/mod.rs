// Configuration management adapter
// This module handles application configuration loading and management

pub mod loader;
pub mod settings;

pub use loader::ConfigLoader;
pub use settings::AppSettings;
