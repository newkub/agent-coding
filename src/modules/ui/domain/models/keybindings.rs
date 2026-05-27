use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyBindings {
    pub bindings: HashMap<String, KeyBinding>,
    pub mode: KeyBindingMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBinding {
    pub key: String,
    pub action: String,
    pub description: String,
    pub context: KeyContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub enum KeyBindingMode {
    #[default]
    Normal,
    Vim,
    Emacs,
    Custom,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyContext {
    Global,
    GitTab,
    FilesTab,
    AgentTab,
    TerminalTab,
    CommandPalette,
}

impl KeyBindings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_default_bindings() -> Self {
        let mut bindings = Self::new();
        
        // Global bindings
        bindings.bindings.insert("Tab".to_string(), KeyBinding {
            key: "Tab".to_string(),
            action: "next_tab".to_string(),
            description: "Switch to next tab".to_string(),
            context: KeyContext::Global,
        });
        
        bindings.bindings.insert("Shift+Tab".to_string(), KeyBinding {
            key: "Shift+Tab".to_string(),
            action: "prev_tab".to_string(),
            description: "Switch to previous tab".to_string(),
            context: KeyContext::Global,
        });
        
        bindings.bindings.insert("Ctrl+K".to_string(), KeyBinding {
            key: "Ctrl+K".to_string(),
            action: "command_palette".to_string(),
            description: "Open command palette".to_string(),
            context: KeyContext::Global,
        });
        
        bindings.bindings.insert("q".to_string(), KeyBinding {
            key: "q".to_string(),
            action: "quit".to_string(),
            description: "Quit application".to_string(),
            context: KeyContext::Global,
        });
        
        // Vim-style bindings
        bindings.bindings.insert("j".to_string(), KeyBinding {
            key: "j".to_string(),
            action: "down".to_string(),
            description: "Move down".to_string(),
            context: KeyContext::GitTab,
        });
        
        bindings.bindings.insert("k".to_string(), KeyBinding {
            key: "k".to_string(),
            action: "up".to_string(),
            description: "Move up".to_string(),
            context: KeyContext::GitTab,
        });
        
        bindings.bindings.insert("h".to_string(), KeyBinding {
            key: "h".to_string(),
            action: "left".to_string(),
            description: "Move left".to_string(),
            context: KeyContext::GitTab,
        });
        
        bindings.bindings.insert("l".to_string(), KeyBinding {
            key: "l".to_string(),
            action: "right".to_string(),
            description: "Move right".to_string(),
            context: KeyContext::GitTab,
        });
        
        bindings.bindings.insert("g".to_string(), KeyBinding {
            key: "g".to_string(),
            action: "goto_top".to_string(),
            description: "Go to top".to_string(),
            context: KeyContext::GitTab,
        });
        
        bindings.bindings.insert("G".to_string(), KeyBinding {
            key: "G".to_string(),
            action: "goto_bottom".to_string(),
            description: "Go to bottom".to_string(),
            context: KeyContext::GitTab,
        });
        
        bindings.bindings.insert("dd".to_string(), KeyBinding {
            key: "dd".to_string(),
            action: "delete_line".to_string(),
            description: "Delete line".to_string(),
            context: KeyContext::GitTab,
        });
        
        bindings.bindings.insert("yy".to_string(), KeyBinding {
            key: "yy".to_string(),
            action: "yank_line".to_string(),
            description: "Copy line".to_string(),
            context: KeyContext::GitTab,
        });
        
        bindings.bindings.insert("p".to_string(), KeyBinding {
            key: "p".to_string(),
            action: "paste".to_string(),
            description: "Paste".to_string(),
            context: KeyContext::GitTab,
        });
        
        bindings.bindings.insert("u".to_string(), KeyBinding {
            key: "u".to_string(),
            action: "undo".to_string(),
            description: "Undo".to_string(),
            context: KeyContext::GitTab,
        });
        
        bindings.bindings.insert("Ctrl+r".to_string(), KeyBinding {
            key: "Ctrl+r".to_string(),
            action: "redo".to_string(),
            description: "Redo".to_string(),
            context: KeyContext::GitTab,
        });
        
        bindings
    }

    pub fn get_vim_bindings() -> Self {
        let mut bindings = Self::get_default_bindings();
        bindings.mode = KeyBindingMode::Vim;
        bindings
    }

    pub fn add_binding(&mut self, key: String, binding: KeyBinding) {
        self.bindings.insert(key, binding);
    }

    pub fn remove_binding(&mut self, key: &str) {
        self.bindings.remove(key);
    }

    pub fn get_binding(&self, key: &str) -> Option<&KeyBinding> {
        self.bindings.get(key)
    }

    pub fn set_mode(&mut self, mode: KeyBindingMode) {
        self.mode = mode;
    }
}
