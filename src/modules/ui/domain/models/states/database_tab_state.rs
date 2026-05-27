use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct DatabaseTabState {
    pub connections: Vec<DatabaseConnection>,
    pub selected_connection_index: usize,
    pub tables: Vec<String>,
    pub selected_table: Option<String>,
    pub query: String,
    pub results: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConnection {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub is_connected: bool,
}

impl TabState for DatabaseTabState {
    fn tab(&self) -> Tab {
        Tab::Database
    }
}
