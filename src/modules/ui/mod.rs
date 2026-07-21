// UI module - Clean Architecture (FP-style)
pub mod application;
pub mod domain;
pub(crate) mod ports;
pub mod types;

pub use domain::*;
