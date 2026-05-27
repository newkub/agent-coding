use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MouseState {
    pub is_enabled: bool,
    pub position: Option<MousePosition>,
    pub drag_state: Option<DragState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MousePosition {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragState {
    pub start_x: u16,
    pub start_y: u16,
    pub current_x: u16,
    pub current_y: u16,
    pub item_type: DragItemType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DragItemType {
    Commit,
    File,
    Branch,
    Stash,
}

impl MouseState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
        self.position = None;
        self.drag_state = None;
    }

    pub fn update_position(&mut self, x: u16, y: u16) {
        self.position = Some(MousePosition { x, y });
    }

    pub fn start_drag(&mut self, x: u16, y: u16, item_type: DragItemType) {
        self.drag_state = Some(DragState {
            start_x: x,
            start_y: y,
            current_x: x,
            current_y: y,
            item_type,
        });
    }

    pub fn update_drag(&mut self, x: u16, y: u16) {
        if let Some(ref mut drag) = self.drag_state {
            drag.current_x = x;
            drag.current_y = y;
        }
    }

    pub fn end_drag(&mut self) -> Option<DragState> {
        self.drag_state.take()
    }

    pub const fn is_dragging(&self) -> bool {
        self.drag_state.is_some()
    }
}
