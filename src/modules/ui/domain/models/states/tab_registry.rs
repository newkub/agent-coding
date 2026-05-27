use std::collections::HashMap;
use std::sync::RwLock;

use crate::shared::kernel::types::Tab;
use super::TabState;

/// Registry that holds all tab states with type-safe access
pub(crate) struct TabStateRegistry {
    states: RwLock<HashMap<Tab, Box<dyn TabState>>>,
}

impl TabStateRegistry {
    pub(crate) fn new() -> Self {
        Self {
            states: RwLock::new(HashMap::new()),
        }
    }

    pub(crate) fn register<T: TabState + 'static>(&self, state: T) {
        let tab = state.tab();
        let mut states = self.states.write().unwrap();
        states.insert(tab, Box::new(state));
    }

    pub(crate) fn get_tab(&self, tab: Tab) -> Option<Box<dyn TabState>> {
        let states = self.states.read().unwrap();
        states.get(&tab).cloned()
    }

    pub(crate) fn tabs(&self) -> Vec<Tab> {
        let states = self.states.read().unwrap();
        states.keys().cloned().collect()
    }
}

impl Default for TabStateRegistry {
    fn default() -> Self {
        Self::new()
    }
}