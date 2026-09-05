use super::TabColumns;
use crate::modules::ui::domain::models::AppState;

/// Render Packages tab columns — backed by `packages_tab_state`
/// (dependencies parsed from the project manifest)
pub(crate) fn render_packages_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.packages_tab_state;

    let filtered: Vec<&crate::modules::ui::domain::models::PackageItem> = tab_state
        .packages
        .iter()
        .filter(|p| !tab_state.show_outdated || p.outdated)
        .collect();

    let left = {
        let items: Vec<String> = filtered
            .iter()
            .take(20)
            .enumerate()
            .map(|(i, p)| {
                let marker = if i == tab_state.selected_package_index {
                    ">"
                } else {
                    " "
                };
                format!("{marker} {} ({})", p.name, p.version)
            })
            .collect();
        if items.is_empty() {
            "(no packages found)".to_string()
        } else {
            items.join("\n")
        }
    };

    let center = match filtered.get(tab_state.selected_package_index) {
        Some(p) => format!(
            "Package: {}\nVersion: {}\nCategory: {}\nOutdated: {}",
            p.name, p.version, p.category, p.outdated
        ),
        None => "No package selected".to_string(),
    };

    let right = format!(
        "Manager: {}\nPackages: {}\nOutdated only: {}\n\n[r] Refresh  [o] Toggle outdated",
        if tab_state.package_manager.is_empty() {
            "-"
        } else {
            tab_state.package_manager.as_str()
        },
        tab_state.packages.len(),
        tab_state.show_outdated,
    );

    TabColumns::new(left, center, right)
}

/// Render Snippets tab columns (shared by Snippet and Snippets tabs)
pub(crate) fn render_snippets_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.snippet_tab_state;

    let left = {
        let items: Vec<String> = tab_state
            .snippets
            .iter()
            .enumerate()
            .map(|(i, s)| {
                let marker = if i == tab_state.selected_snippet_index {
                    ">"
                } else {
                    " "
                };
                format!("{marker} {} ({})", s.name, s.language)
            })
            .collect();
        if items.is_empty() {
            "(no snippets — Ctrl+K → New Snippet)".to_string()
        } else {
            items.join("\n")
        }
    };

    let center = if tab_state.is_editing {
        format!("Editing:\n{}", tab_state.edit_content)
    } else {
        match tab_state.snippets.get(tab_state.selected_snippet_index) {
            Some(s) => format!("{}\n\n{}", s.name, s.code),
            None => "No snippet selected".to_string(),
        }
    };

    let right = format!(
        "Snippets: {}\nCategory: {}",
        tab_state.snippets.len(),
        if tab_state.selected_category.is_empty() {
            "all"
        } else {
            tab_state.selected_category.as_str()
        },
    );

    TabColumns::new(left, center, right)
}

/// Render Skills tab columns — backed by `skills_tab_state`
/// (subagents registered in the subagent manager)
pub(crate) fn render_skills_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.skills_tab_state;

    let left = {
        let items: Vec<String> = tab_state
            .skills
            .iter()
            .take(20)
            .enumerate()
            .map(|(i, s)| {
                let marker = if i == tab_state.selected_skill_index {
                    ">"
                } else {
                    " "
                };
                format!("{marker} {} [{}]", s.name, s.status)
            })
            .collect();
        if items.is_empty() {
            "(no skills registered)".to_string()
        } else {
            items.join("\n")
        }
    };

    let center = match tab_state.skills.get(tab_state.selected_skill_index) {
        Some(s) => format!(
            "Skill: {}\nStatus: {}\n\n{}",
            s.name, s.status, s.description
        ),
        None => "No skill selected".to_string(),
    };

    let right = format!(
        "Source: {}\nSkills: {}\n\n[Enter] Load  [Ctrl+K → Run] Execute",
        tab_state.skill_source_filter.as_deref().unwrap_or("all"),
        tab_state.skills.len(),
    );

    TabColumns::new(left, center, right)
}

/// Render Workflows tab columns — backed by `workflows_tab_state`
pub(crate) fn render_workflows_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.workflows_tab_state;

    let left = {
        let items: Vec<String> = tab_state
            .workflows
            .iter()
            .enumerate()
            .map(|(i, w)| {
                let marker = if i == tab_state.selected_workflow_index {
                    ">"
                } else {
                    " "
                };
                format!("{marker} {} [{}]", w.name, w.status)
            })
            .collect();
        if items.is_empty() {
            "(no workflows)".to_string()
        } else {
            items.join("\n")
        }
    };

    let center = if tab_state.is_editing {
        "Editing workflow…".to_string()
    } else {
        match tab_state.workflows.get(tab_state.selected_workflow_index) {
            Some(w) => format!("{}\n\nSteps:\n{}", w.name, w.steps.join("\n")),
            None => "No workflow selected".to_string(),
        }
    };

    let right = format!(
        "Status: {}\nWorkflows: {}\n\n[Ctrl+K → Run] Execute",
        tab_state.execution_status.as_deref().unwrap_or("idle"),
        tab_state.workflows.len(),
    );

    TabColumns::new(left, center, right)
}

/// Render Collaboration tab columns — backed by `collaboration_tab_state`
/// (sessions/messages from the collaboration repository)
pub(crate) fn render_collaboration_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.collaboration_tab_state;

    let left = {
        let items: Vec<String> = tab_state
            .sessions
            .iter()
            .take(20)
            .enumerate()
            .map(|(i, s)| {
                let marker = if i == tab_state.selected_session_index {
                    ">"
                } else {
                    " "
                };
                format!(
                    "{marker} {} ({:?}, {} peers)",
                    s.name,
                    s.status,
                    s.participants.len()
                )
            })
            .collect();
        if items.is_empty() {
            "(no sessions — Ctrl+K → New Session)".to_string()
        } else {
            items.join("\n")
        }
    };

    let center = if tab_state.messages.is_empty() {
        if tab_state.input.is_empty() {
            "(no messages — join a session, then type and press Enter)".to_string()
        } else {
            format!("Input:\n> {}", tab_state.input)
        }
    } else {
        let history = tab_state
            .messages
            .iter()
            .rev()
            .take(30)
            .rev()
            .map(|m| {
                format!(
                    "[{}] {}: {}",
                    m.timestamp.format("%H:%M:%S"),
                    m.sender_id.as_str(),
                    m.content
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        if tab_state.input.is_empty() {
            history
        } else {
            format!("{history}\n\n> {}", tab_state.input)
        }
    };

    let right = {
        let participants = tab_state
            .sessions
            .get(tab_state.selected_session_index)
            .map(|s| {
                s.participants
                    .iter()
                    .map(|p| {
                        format!(
                            "  {} {:?}{}",
                            p.name,
                            p.role,
                            if p.is_online { " (online)" } else { "" }
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .unwrap_or_else(|| "  (none)".to_string());
        format!(
            "Joined: {}\nHost: {}\n\nParticipants:\n{}\n\n[Enter] Select/Send\n[Ctrl+K] Actions",
            if state.collaboration_state.is_active {
                state
                    .collaboration_state
                    .session_id
                    .as_deref()
                    .unwrap_or("yes")
            } else {
                "no"
            },
            tab_state.is_host,
            participants,
        )
    };

    TabColumns::new(left, center, right)
}

/// Render Macros tab columns — backed by `macros_tab_state`
/// (macros from the macro repository)
pub(crate) fn render_macros_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.macros_tab_state;

    let left = {
        let items: Vec<String> = tab_state
            .macros
            .iter()
            .take(20)
            .enumerate()
            .map(|(i, m)| {
                let marker = if i == tab_state.selected_index {
                    ">"
                } else {
                    " "
                };
                format!(
                    "{marker} {} ({} steps, {} uses)",
                    m.name,
                    m.step_count(),
                    m.usage_count
                )
            })
            .collect();
        if items.is_empty() {
            "(no macros — type a name, Enter records)".to_string()
        } else {
            items.join("\n")
        }
    };

    let center = match tab_state.macros.get(tab_state.selected_index) {
        Some(m) => {
            let steps = m
                .steps
                .iter()
                .enumerate()
                .map(|(i, s)| format!("  {}. {:?}", i + 1, s))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "{}\n{}\n\nSteps:\n{}",
                m.name,
                if m.description.is_empty() {
                    "(no description)"
                } else {
                    m.description.as_str()
                },
                if steps.is_empty() {
                    "  (empty)".to_string()
                } else {
                    steps
                },
            )
        }
        None => "No macro selected".to_string(),
    };

    let right = format!(
        "Recording: {}\nStatus: {}\nInput: {}\n\n[Enter] Record/Stop/Playback\n[Ctrl+K] Actions",
        if tab_state.recording {
            tab_state.recording_id.as_deref().unwrap_or("starting…")
        } else {
            "off"
        },
        tab_state.status.as_deref().unwrap_or("idle"),
        tab_state.input,
    );

    TabColumns::new(left, center, right)
}

/// Render Settings tab columns — shows the real loaded settings
pub(crate) fn render_settings_tab(state: &AppState) -> TabColumns {
    let tab_state = &state.settings_tab_state;

    let left = format!("Category: {}", tab_state.selected_category_index);
    let center = format!(
        "Theme: {}\nFont size: {}\n\n(settings loaded from config.toml)",
        if tab_state.theme.is_empty() {
            "default"
        } else {
            tab_state.theme.as_str()
        },
        tab_state.font_size,
    );
    let right = "Keys:\n  [?] Help\n  [q] Quit\n\n[Ctrl+K → Apply] Save".to_string();

    TabColumns::new(left, center, right)
}
