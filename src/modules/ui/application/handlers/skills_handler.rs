use super::tab_action_types::TabAction;
use crate::modules::ui::domain::models::AppState;
use crate::shared::kernel::result::AppResult;

/// Skills tab action handler
pub(crate) fn handle_skills_action(state: &mut AppState, action: TabAction) -> AppResult<()> {
    match action {
        TabAction::LoadSkill => {
            if let Some(skill) = state
                .skills_tab_state
                .skills
                .get_mut(state.skills_tab_state.selected_skill_index)
            {
                skill.status = "loaded".to_string();
            }
        }
        TabAction::RunSkill => {
            if let Some(skill) = state
                .skills_tab_state
                .skills
                .get_mut(state.skills_tab_state.selected_skill_index)
            {
                skill.status = "running".to_string();
            }
        }
        _ => {}
    }
    Ok(())
}
