use std::fmt;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    Io(String),
    Render(String),
    Input(String),
    State(String),
    NotFound(String),
    PermissionDenied(String),
    ValidationError(String),
    Database(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "IO Error: {}", msg),
            Self::Render(msg) => write!(f, "Render Error: {}", msg),
            Self::Input(msg) => write!(f, "Input Error: {}", msg),
            Self::State(msg) => write!(f, "State Error: {}", msg),
            Self::NotFound(msg) => write!(f, "Not Found: {}", msg),
            Self::PermissionDenied(msg) => write!(f, "Permission Denied: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            Self::Database(msg) => write!(f, "Database Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<git2::Error> for AppError {
    fn from(err: git2::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err.to_string())
    }
}
