use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct PackagesTabState {
    pub ecosystems: Vec<PackageEcosystem>,
    pub selected_ecosystem_index: usize,
    pub packages: Vec<PackageInfo>,
    pub selected_package_index: usize,
    pub command_input: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageEcosystem {
    pub id: String,
    pub name: String,
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub installed: bool,
}

impl TabState for PackagesTabState {
    fn tab(&self) -> Tab {
        Tab::Packages
    }
}
