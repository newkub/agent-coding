use super::TabColumns;
use crate::modules::ui::domain::models::AppState;

/// Render Packages tab columns
pub(crate) fn render_packages_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.packages_tab_state;

    let left = format!(
        "Selected: {}\nOutdated only: {}",
        tab_state.selected_package_index, tab_state.show_outdated
    );
    let center = "No package selected".to_string();
    let right = "[i] Install\n[u] Uninstall\n[o] Toggle outdated".to_string();

    TabColumns::new(left, center, right)
}

/// Render Snippets tab columns (shared by Snippet and Snippets tabs)
pub(crate) fn render_snippets_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.snippet_tab_state;

    let left = format!(
        "Category: {}\nSelected: {}",
        if tab_state.selected_category.is_empty() {
            "all"
        } else {
            tab_state.selected_category.as_str()
        },
        tab_state.selected_snippet_index,
    );

    let center = if tab_state.is_editing {
        format!("Editing:\n{}", tab_state.edit_content)
    } else {
        "No snippet selected".to_string()
    };

    let right = "Tags: -".to_string();

    TabColumns::new(left, center, right)
}

/// Render Skills tab columns
pub(crate) fn render_skills_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.skills_tab_state;

    let left = format!(
        "Source: {}\nSelected: {}",
        tab_state.skill_source_filter.as_deref().unwrap_or("all"),
        tab_state.selected_skill_index,
    );
    let center = "No skill selected".to_string();
    let right = "Progress: -".to_string();

    TabColumns::new(left, center, right)
}

/// Render Workflows tab columns
pub(crate) fn render_workflows_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.workflows_tab_state;

    let left = format!("Selected: {}", tab_state.selected_workflow_index);
    let center = if tab_state.is_editing {
        "Editing workflow…".to_string()
    } else {
        "No workflow selected".to_string()
    };
    let right = format!(
        "Status: {}",
        tab_state.execution_status.as_deref().unwrap_or("idle")
    );

    TabColumns::new(left, center, right)
}

/// Render Settings tab columns
pub(crate) fn render_settings_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.settings_tab_state;

    let left = format!("Category: {}", tab_state.selected_category_index);
    let center = format!(
        "Theme: {}\nFont size: {}",
        if tab_state.theme.is_empty() {
            "default"
        } else {
            tab_state.theme.as_str()
        },
        tab_state.font_size,
    );
    let right = "Keys:\n  [?] Help\n  [q] Quit".to_string();

    TabColumns::new(left, center, right)
}
