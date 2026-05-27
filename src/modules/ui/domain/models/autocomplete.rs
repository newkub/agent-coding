use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AutocompleteState {
    pub is_active: bool,
    pub trigger_char: Option<char>, // '@' for users, '#' for tags
    pub query: String,
    pub suggestions: Vec<Suggestion>,
    pub selected_index: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub id: String,
    pub label: String,
    pub description: String,
    pub kind: SuggestionKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuggestionKind {
    User,
    Tag,
    Command,
}

impl AutocompleteState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn activate(&mut self, trigger_char: char) {
        self.is_active = true;
        self.trigger_char = Some(trigger_char);
        self.query = String::new();
        self.selected_index = 0;
        self.suggestions = Self::get_default_suggestions(trigger_char);
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.trigger_char = None;
        self.query = String::new();
        self.suggestions.clear();
        self.selected_index = 0;
    }

    pub fn update_query(&mut self, query: String) {
        self.query = query.clone();
        self.suggestions = self.filter_suggestions(&query);
        self.selected_index = 0;
    }

    pub fn select_next(&mut self) {
        if !self.suggestions.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.suggestions.len();
        }
    }

    pub fn select_prev(&mut self) {
        if !self.suggestions.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.suggestions.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn get_selected(&self) -> Option<&Suggestion> {
        self.suggestions.get(self.selected_index)
    }

    fn get_default_suggestions(trigger_char: char) -> Vec<Suggestion> {
        match trigger_char {
            '@' => vec![
                Suggestion {
                    id: "user1".to_string(),
                    label: "@john".to_string(),
                    description: "John Doe".to_string(),
                    kind: SuggestionKind::User,
                },
                Suggestion {
                    id: "user2".to_string(),
                    label: "@jane".to_string(),
                    description: "Jane Smith".to_string(),
                    kind: SuggestionKind::User,
                },
                Suggestion {
                    id: "user3".to_string(),
                    label: "@bob".to_string(),
                    description: "Bob Johnson".to_string(),
                    kind: SuggestionKind::User,
                },
            ],
            '#' => vec![
                Suggestion {
                    id: "tag1".to_string(),
                    label: "#bug".to_string(),
                    description: "Bug report".to_string(),
                    kind: SuggestionKind::Tag,
                },
                Suggestion {
                    id: "tag2".to_string(),
                    label: "#feature".to_string(),
                    description: "Feature request".to_string(),
                    kind: SuggestionKind::Tag,
                },
                Suggestion {
                    id: "tag3".to_string(),
                    label: "#urgent".to_string(),
                    description: "Urgent priority".to_string(),
                    kind: SuggestionKind::Tag,
                },
            ],
            _ => vec![],
        }
    }

    fn filter_suggestions(&self, query: &str) -> Vec<Suggestion> {
        let default = match self.trigger_char {
            Some('@') => Self::get_default_suggestions('@'),
            Some('#') => Self::get_default_suggestions('#'),
            _ => vec![],
        };

        if query.is_empty() {
            return default;
        }

        let query_lower = query.to_lowercase();
        default
            .into_iter()
            .filter(|s| {
                s.label.to_lowercase().contains(&query_lower)
                    || s.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
}
