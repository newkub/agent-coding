impl super::AppState {
    // Legacy state methods for backward compatibility

    pub fn start_diff_review(&mut self, diff_text: String) {
        self.diff_state.is_active = true;
        self.diff_state.diff_text = diff_text;
    }

    pub fn end_diff_review(&mut self) {
        self.diff_state.is_active = false;
        self.diff_state.diff_text = String::new();
    }

    pub fn open_snippet_manager(&mut self) {
        self.snippet_state.is_active = true;
    }

    pub fn close_snippet_manager(&mut self) {
        self.snippet_state.is_active = false;
    }

    pub fn start_macro_recording(&mut self, macro_id: String) {
        self.macro_state.is_recording = true;
        self.macro_state.current_macro_id = Some(macro_id);
    }

    pub fn stop_macro_recording(&mut self) {
        self.macro_state.is_recording = false;
        self.macro_state.current_macro_id = None;
    }

    pub fn preview_command(&mut self, _command: &str) {
        self.sandbox_state.is_preview_active = true;
        // command_preview is of type CommandPreview, not String
        // For now, just set the preview flag
    }

    pub fn hide_command_preview(&mut self) {
        self.sandbox_state.is_preview_active = false;
        self.sandbox_state.command_preview = None;
    }

    pub fn open_metrics(&mut self) {
        self.metrics_state.is_active = true;
    }

    pub fn close_metrics(&mut self) {
        self.metrics_state.is_active = false;
    }

    pub fn join_collaboration(&mut self, session_id: String) {
        self.collaboration_state.session_id = Some(session_id);
        self.collaboration_state.is_active = true;
    }

    pub fn leave_collaboration(&mut self) {
        self.collaboration_state.session_id = None;
        self.collaboration_state.is_active = false;
    }
}
