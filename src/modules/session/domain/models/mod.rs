use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Session ID type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SessionId(pub String);

impl SessionId {
    // Pure constructor - moved to application layer for ID generation
    pub const fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Message role in conversation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

impl MessageRole {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::System => "system",
            Self::User => "user",
            Self::Assistant => "assistant",
            Self::Tool => "tool",
        }
    }
}

/// A single message in the session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub role: MessageRole,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub metadata: Option<MessageMetadata>,
}

/// Optional metadata for messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MessageMetadata {
    pub model: Option<String>,
    pub tokens_used: Option<u32>,
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// Tool call metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub name: String,
    pub arguments: serde_json::Value,
    pub result: Option<String>,
}

impl Message {
    // Pure constructor - timestamp and ID moved to application layer
    pub const fn create(
        id: String,
        role: MessageRole,
        content: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            role,
            content,
            created_at,
            metadata: None,
        }
    }
}

/// Session represents a conversation context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub name: String,
    pub messages: Vec<Message>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: SessionMetadata,
}

/// Session metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub provider: Option<String>,
    pub model: Option<String>,
    pub tags: Vec<String>,
    pub pinned: bool,
}

impl Session {
    // Pure constructor - timestamp and ID moved to application layer
    pub fn create(
        id: SessionId,
        name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            messages: Vec::new(),
            created_at,
            updated_at,
            metadata: SessionMetadata::default(),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    pub fn total_tokens(&self) -> u32 {
        self.messages
            .iter()
            .filter_map(|m| m.metadata.as_ref())
            .filter_map(|m| m.tokens_used)
            .sum()
    }
}