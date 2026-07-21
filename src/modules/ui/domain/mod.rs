// Domain layer - pure business logic
pub mod events;
pub mod models;
pub mod operations;
pub mod validators;

// Re-exports
pub use events::UIEvent;
pub use operations::{
    calculate_next_column, calculate_next_tab, calculate_prev_column, calculate_prev_tab,
};
