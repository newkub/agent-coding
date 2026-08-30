//! Ratatui UI adapter

mod adapter;
mod types;

#[cfg(test)]
mod tests;

pub(crate) use self::adapter::render_app_state;
pub(crate) use self::types::{RATerminal, RatatuiAdapter};
