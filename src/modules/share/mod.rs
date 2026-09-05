// Share module - Clean Architecture (FP-style)
// Contains unimplemented application code; allow dead code until fully wired.
#![allow(dead_code)]
pub(crate) mod application;
pub mod domain;
pub(crate) mod ports;
pub mod types;
