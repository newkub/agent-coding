use crate::modules::session::domain::models::Session;

/// Service: Filter sessions by criteria
pub(crate) fn filter_sessions(
    sessions: &[Session],
    predicate: impl Fn(&Session) -> bool,
) -> Vec<Session> {
    sessions.iter().filter(|s| predicate(s)).cloned().collect()
}

/// Service: Sort sessions by criteria
pub(crate) fn sort_sessions(sessions: Vec<Session>, by: SortCriteria) -> Vec<Session> {
    let mut sorted = sessions;
    match by {
        SortCriteria::Name => sorted.sort_by_key(|a| a.name.clone()),
        SortCriteria::CreatedAt => sorted.sort_by_key(|a| std::cmp::Reverse(a.created_at)),
        SortCriteria::UpdatedAt => sorted.sort_by_key(|a| std::cmp::Reverse(a.updated_at)),
        SortCriteria::MessageCount => sorted.sort_by_key(|a| std::cmp::Reverse(a.messages.len())),
    }
    sorted
}

#[derive(Debug, Clone)]
pub(crate) enum SortCriteria {
    Name,
    CreatedAt,
    UpdatedAt,
    MessageCount,
}

/// Service: Search sessions by name
pub(crate) fn search_sessions(sessions: &[Session], query: &str) -> Vec<Session> {
    let query_lower = query.to_lowercase();
    sessions
        .iter()
        .filter(|s| s.name.to_lowercase().contains(&query_lower))
        .cloned()
        .collect()
}

/// Service: Group sessions by metadata
pub(crate) fn group_sessions_by_model(
    sessions: &[Session],
) -> std::collections::HashMap<String, Vec<Session>> {
    let mut groups: std::collections::HashMap<String, Vec<Session>> =
        std::collections::HashMap::new();
    for session in sessions {
        let model = session
            .metadata
            .model
            .clone()
            .unwrap_or_else(|| "unknown".to_string());
        groups.entry(model).or_default().push(session.clone());
    }
    groups
}
