// Git module - Clean Architecture (FP-style)
#![allow(dead_code)]
pub mod domain;
pub(crate) mod application;
pub(crate) mod ports;
pub mod types;

pub use domain::*;
