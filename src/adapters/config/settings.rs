use serde::{Deserialize, Serialize};

/// Application settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct AppSettings {
    /// UI settings
    pub ui: UISettings,
    /// AI settings
    pub ai: AISettings,
    /// Git settings
    pub git: GitSettings,
    /// Terminal settings
    pub terminal: TerminalSettings,
    /// Memory settings
    pub memory: MemorySettings,
    /// Security settings
    pub security: SecuritySettings,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UISettings {
    /// Theme name
    pub theme: String,
    /// Font size
    pub font_size: u8,
    /// Show line numbers
    pub show_line_numbers: bool,
    /// Enable mouse support
    pub enable_mouse: bool,
    /// Accessibility settings
    pub accessibility: AccessibilitySettings,
}

/// Accessibility configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilitySettings {
    /// Enable screen reader support
    pub enable_screen_reader: bool,
    /// Enable high contrast mode
    pub high_contrast: bool,
    /// Theme name (default, high-contrast, dark)
    pub theme: String,
    /// Enable reduced motion
    pub reduced_motion: bool,
    /// Announce focus changes
    pub announce_focus: bool,
    /// Announce content changes
    pub announce_content: bool,
    /// Font size multiplier (1.0 = default, 1.2 = 20% larger)
    pub font_scale: f32,
}

/// AI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISettings {
    /// Default model
    pub default_model: String,
    /// API endpoint
    pub api_endpoint: String,
    /// Maximum tokens
    pub max_tokens: u32,
    /// Temperature
    pub temperature: f32,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Enable response streaming
    pub enable_streaming: bool,
    /// Enable response caching
    pub enable_caching: bool,
    /// Cache TTL in seconds
    pub cache_ttl: u64,
}

/// Git configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitSettings {
    /// Default branch name
    pub default_branch: String,
    /// Auto-push after commit
    pub auto_push: bool,
    /// Sign commits with GPG
    pub sign_commits: bool,
}

/// Terminal configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalSettings {
    /// Default shell
    pub shell: String,
    /// Enable ANSI colors
    pub enable_ansi: bool,
    /// History size
    pub history_size: usize,
}

/// Memory configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySettings {
    /// Maximum cache size in MB
    pub max_cache_size_mb: u64,
    /// Enable memory pooling
    pub enable_memory_pooling: bool,
    /// Session history limit
    pub session_history_limit: usize,
    /// Enable lazy loading
    pub enable_lazy_loading: bool,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    /// Enable database encryption
    pub enable_encryption: bool,
    /// Encryption password (should be loaded from env var in production)
    pub encryption_password: Option<String>,
    /// Enable audit logging
    pub enable_audit_logging: bool,
    /// Enable command sandboxing
    pub enable_sandbox: bool,
    /// Mask sensitive data in logs
    pub mask_sensitive_data: bool,
}


impl Default for UISettings {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            font_size: 12,
            show_line_numbers: true,
            enable_mouse: true,
            accessibility: AccessibilitySettings::default(),
        }
    }
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            enable_screen_reader: false,
            high_contrast: false,
            theme: "default".to_string(),
            reduced_motion: false,
            announce_focus: true,
            announce_content: true,
            font_scale: 1.0,
        }
    }
}

impl Default for AISettings {
    fn default() -> Self {
        Self {
            default_model: "gpt-4".to_string(),
            api_endpoint: "https://api.openai.com/v1".to_string(),
            max_tokens: 4096,
            temperature: 0.7,
            request_timeout: 30,
            enable_streaming: true,
            enable_caching: true,
            cache_ttl: 3600,
        }
    }
}

impl Default for GitSettings {
    fn default() -> Self {
        Self {
            default_branch: "main".to_string(),
            auto_push: false,
            sign_commits: false,
        }
    }
}

impl Default for TerminalSettings {
    fn default() -> Self {
        Self {
            shell: "bash".to_string(),
            enable_ansi: true,
            history_size: 1000,
        }
    }
}

impl Default for MemorySettings {
    fn default() -> Self {
        Self {
            max_cache_size_mb: 50,
            enable_memory_pooling: true,
            session_history_limit: 100,
            enable_lazy_loading: true,
        }
    }
}

impl Default for SecuritySettings {
    fn default() -> Self {
        Self {
            enable_encryption: false, // Disabled by default for backward compatibility
            encryption_password: None, // Should be loaded from env var AGENT_TUI_ENCRYPTION_KEY
            enable_audit_logging: true,
            enable_sandbox: true,
            mask_sensitive_data: true,
        }
    }
}
