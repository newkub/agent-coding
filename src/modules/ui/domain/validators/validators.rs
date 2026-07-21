use crate::shared::types::Tab;

/// Validate tab index is within bounds
pub(crate) const fn validate_tab(tab: Tab) -> bool {
    matches!(
        tab,
        Tab::Agent
            | Tab::Git
            | Tab::Cli
            | Tab::Snippet
            | Tab::Skills
            | Tab::Workflows
            | Tab::Files
            | Tab::Settings
    )
}
