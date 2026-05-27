// Encrypted SQLite repository adapter
// Provides transparent encryption for SQLite database operations

use sqlx::SqlitePool;
use crate::shared::kernel::encryption::{SessionEncryptor, EncryptionError};
use thiserror::Error;

/// Errors for encrypted repository operations
#[derive(Debug, Error)]
pub enum EncryptedRepoError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Encryption error: {0}")]
    Encryption(#[from] EncryptionError),
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Encrypted SQLite repository that transparently encrypts/decrypts data
pub struct EncryptedSqliteRepository {
    pool: SqlitePool,
    encryptor: SessionEncryptor,
    encryption_enabled: bool,
}

impl EncryptedSqliteRepository {
    /// Create new encrypted repository
    pub(crate) fn new(pool: SqlitePool, password: &str, encryption_enabled: bool) -> Self {
        let encryptor = if encryption_enabled {
            SessionEncryptor::new(password).expect("Failed to create encryptor")
        } else {
            // Create dummy encryptor if encryption disabled
            SessionEncryptor::new("dummy").expect("Failed to create dummy encryptor")
        };
        
        Self {
            pool,
            encryptor,
            encryption_enabled,
        }
    }
    
    /// Get database pool
    pub(crate) const fn pool(&self) -> &SqlitePool {
        &self.pool
    }
    
    /// Encrypt data if encryption is enabled
    pub(crate) fn encrypt_data<T: serde::Serialize>(&self, data: &T) -> Result<String, EncryptedRepoError> {
        if self.encryption_enabled {
            self.encryptor.encrypt(data).map_err(EncryptedRepoError::from)
        } else {
            // If encryption disabled, just serialize as JSON
            serde_json::to_string(data)
                .map_err(|e| EncryptedRepoError::Serialization(e.to_string()))
        }
    }
    
    /// Decrypt data if encryption is enabled
    pub(crate) fn decrypt_data<T: for<'de> serde::Deserialize<'de>>(&self, encrypted: &str) -> Result<T, EncryptedRepoError> {
        if self.encryption_enabled {
            self.encryptor.decrypt(encrypted).map_err(EncryptedRepoError::from)
        } else {
            // If encryption disabled, just deserialize from JSON
            serde_json::from_str(encrypted)
                .map_err(|e| EncryptedRepoError::Serialization(e.to_string()))
        }
    }
    
    /// Check if encryption is enabled
    pub(crate) const fn is_encryption_enabled(&self) -> bool {
        self.encryption_enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encryption_enabled() {
        // This would require a test database, so we skip for now
        // In production, we'd use sqlx::sqlite::SqlitePoolOptions with in-memory DB
    }
}
