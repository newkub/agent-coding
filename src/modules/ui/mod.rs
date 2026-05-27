// UI module - Clean Architecture (FP-style)
pub mod domain;
pub mod application;
pub(crate) mod ports;
pub mod types;

pub use domain::*;
