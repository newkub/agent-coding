//! Ratatui UI adapter

mod types;
mod adapter;

#[cfg(test)]
mod tests;

pub(crate) use self::types::{RatatuiAdapter, RATerminal};
pub(crate) use self::adapter::render_app_state;
