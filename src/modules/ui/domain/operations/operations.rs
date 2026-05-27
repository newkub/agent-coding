use crate::shared::types::{Column, Tab};

/// Calculate next tab in the cycle
pub const fn calculate_next_tab(current: Tab) -> Tab {
    current.next()
}

/// Calculate previous tab in the cycle
pub const fn calculate_prev_tab(current: Tab) -> Tab {
    current.prev()
}

/// Calculate next column in the cycle
pub const fn calculate_next_column(current: Column) -> Column {
    current.next()
}

/// Calculate previous column in the cycle
pub const fn calculate_prev_column(current: Column) -> Column {
    current.prev()
}