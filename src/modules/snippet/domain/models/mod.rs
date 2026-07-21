use serde::{Deserialize, Serialize};

/// A code snippet with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub id: SnippetId,
    pub name: String,
    pub description: String,
    pub code: String,
    pub language: String,
    pub tags: Vec<String>,
    pub variables: Vec<SnippetVariable>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnippetId(pub String);

impl SnippetId {
    // Pure constructor - moved to application layer for ID generation
    pub const fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SnippetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Variable placeholder in snippet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnippetVariable {
    pub name: String,
    pub description: Option<String>,
    pub default_value: Option<String>,
}

impl Snippet {
    // Pure constructor - timestamp and ID moved to application layer
    pub const fn create(
        id: SnippetId,
        name: String,
        description: String,
        code: String,
        language: String,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            code,
            language,
            tags: Vec::new(),
            variables: Vec::new(),
            created_at,
            updated_at,
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn add_variable(&mut self, name: String, default: Option<String>) {
        self.variables.push(SnippetVariable {
            name,
            description: None,
            default_value: default,
        });
    }

    /// Render snippet with variable substitution
    pub fn render(&self, values: &[(String, String)]) -> String {
        let mut result = self.code.clone();
        for (name, value) in values {
            let placeholder = format!("{{{{{}}}}}", name);
            result = result.replace(&placeholder, value);
        }
        result
    }

    /// Extract variable names from snippet template
    pub fn extract_variables(&self) -> Vec<String> {
        let mut vars = Vec::new();
        let mut chars = self.code.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '{' && chars.peek() == Some(&'{') {
                chars.next(); // consume second {
                let mut var_name = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '}' {
                        chars.next();
                        if chars.peek() == Some(&'}') {
                            chars.next();
                            if !var_name.is_empty() && !vars.contains(&var_name) {
                                vars.push(var_name);
                            }
                            break;
                        }
                    }
                    var_name.push(c);
                    chars.next();
                }
            }
        }

        vars
    }
}

/// Collection of snippets organized by category
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SnippetLibrary {
    pub snippets: Vec<Snippet>,
}

impl SnippetLibrary {
    pub const fn new() -> Self {
        Self {
            snippets: Vec::new(),
        }
    }

    pub fn add(&mut self, snippet: Snippet) {
        self.snippets.push(snippet);
    }

    pub fn search(&self, query: &str) -> Vec<&Snippet> {
        let query_lower = query.to_lowercase();
        self.snippets
            .iter()
            .filter(|s| {
                s.name.to_lowercase().contains(&query_lower)
                    || s.description.to_lowercase().contains(&query_lower)
                    || s.tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn by_language(&self, language: &str) -> Vec<&Snippet> {
        self.snippets
            .iter()
            .filter(|s| s.language.to_lowercase() == language.to_lowercase())
            .collect()
    }

    pub fn by_tag(&self, tag: &str) -> Vec<&Snippet> {
        self.snippets
            .iter()
            .filter(|s| {
                s.tags
                    .iter()
                    .any(|t| t.to_lowercase() == tag.to_lowercase())
            })
            .collect()
    }
}
