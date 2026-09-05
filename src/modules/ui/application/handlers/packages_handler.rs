use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

/// Packages tab action handler
pub(crate) fn handle_packages_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::Install(name) => {
            // Mark the package as installed in the session-local list.
            state.packages_tab_state.packages.push(
                crate::modules::ui::domain::models::PackageItem {
                    name,
                    version: "latest".to_string(),
                    category: "runtime".to_string(),
                    outdated: false,
                },
            );
        }
        TabAction::Uninstall(name) => {
            state.packages_tab_state.packages.retain(|p| p.name != name);
        }
        TabAction::Refresh => {
            // Real reload runs in `tab_effects`; reset the selection here.
            state.packages_tab_state.selected_package_index = 0;
        }
        TabAction::Toggle(_) => {
            state.packages_tab_state.show_outdated = !state.packages_tab_state.show_outdated;
        }
        _ => {}
    }
    Ok(())
}
