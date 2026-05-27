use serde::{Deserialize, Serialize};
use crate::shared::kernel::types::Tab;
use super::TabState;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct ApiTabState {
    pub collections: Vec<ApiCollection>,
    pub selected_collection_index: usize,
    pub request: ApiRequest,
    pub response: String,
    pub environment_variables: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCollection {
    pub id: String,
    pub name: String,
    pub requests: Vec<ApiRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRequest {
    pub id: String,
    pub name: String,
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl TabState for ApiTabState {
    fn tab(&self) -> Tab {
        Tab::Api
    }
}
