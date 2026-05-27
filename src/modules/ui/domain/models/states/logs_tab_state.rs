use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct LogsTabState {
    pub sources: Vec<LogSource>,
    pub selected_source_index: usize,
    pub logs: Vec<LogEntry>,
    pub filter: LogFilter,
    pub auto_scroll: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogSource {
    pub id: String,
    pub name: String,
    pub path: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LogFilter {
    pub level: Option<LogLevel>,
    pub search_query: String,
}

impl TabState for LogsTabState {
    fn tab(&self) -> Tab {
        Tab::Logs
    }
}
