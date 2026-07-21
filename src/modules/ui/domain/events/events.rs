use crate::shared::types::{Column, Tab};

/// UI Domain Events
#[derive(Debug, Clone)]
pub enum UIEvent {
    TabChanged { from: Tab, to: Tab },
    ColumnChanged { from: Column, to: Column },
    FocusToggled { is_focused: bool },
}

impl UIEvent {
    pub const fn tab_changed(from: Tab, to: Tab) -> Self {
        Self::TabChanged { from, to }
    }

    pub const fn column_changed(from: Column, to: Column) -> Self {
        Self::ColumnChanged { from, to }
    }

    pub const fn focus_toggled(is_focused: bool) -> Self {
        Self::FocusToggled { is_focused }
    }
}
