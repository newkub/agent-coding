// Automation module - Clean Architecture (FP-style)
pub mod domain;
pub(crate) mod application;
pub(crate) mod ports;
pub mod types;

pub use domain::*;
