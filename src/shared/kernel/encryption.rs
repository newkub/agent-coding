use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

/// Encryption errors
#[derive(Debug, Error)]
pub enum EncryptionError {
    #[error("Key derivation failed: {0}")]
    KeyDerivationFailed(String),
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Invalid base64: {0}")]
    InvalidBase64(String),
}

/// Session encryptor using AES-256-GCM
pub struct SessionEncryptor {
    cipher: Aes256Gcm,
}

impl SessionEncryptor {
    /// Create new encryptor from password
    pub fn new(password: &str) -> Result<Self, EncryptionError> {
        let key = Self::derive_key(password)?;
        let cipher = Aes256Gcm::new(&key.into());
        Ok(Self { cipher })
    }

    /// Derive 256-bit key from password using PBKDF2-like approach
    fn derive_key(password: &str) -> Result<[u8; 32], EncryptionError> {
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hasher.update(b"agent-tui-salt");
        let result = hasher.finalize();

        let mut key = [0u8; 32];
        key.copy_from_slice(&result[..32]);
        Ok(key)
    }

    /// Encrypt session data
    pub fn encrypt<T: Serialize>(&self, data: &T) -> Result<String, EncryptionError> {
        let json = serde_json::to_vec(data)
            .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;

        let nonce = Nonce::from_slice(b"agent-tui-12");
        let ciphertext = self
            .cipher
            .encrypt(nonce, json.as_ref())
            .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;

        // Combine nonce and ciphertext
        let mut combined = nonce.to_vec();
        combined.extend_from_slice(&ciphertext);

        Ok(STANDARD.encode(&combined))
    }

    /// Decrypt session data
    pub fn decrypt<T: for<'de> Deserialize<'de>>(
        &self,
        encrypted: &str,
    ) -> Result<T, EncryptionError> {
        let combined = STANDARD
            .decode(encrypted)
            .map_err(|e| EncryptionError::InvalidBase64(e.to_string()))?;

        if combined.len() < 12 {
            return Err(EncryptionError::DecryptionFailed(
                "Invalid ciphertext length".to_string(),
            ));
        }

        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?;

        serde_json::from_slice(&plaintext)
            .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestData {
        message: String,
        timestamp: i64,
    }

    #[test]
    fn test_encrypt_decrypt() {
        let encryptor = SessionEncryptor::new("test-password").unwrap();
        let data = TestData {
            message: "Hello, World!".to_string(),
            timestamp: 1234567890,
        };

        let encrypted = encryptor.encrypt(&data).unwrap();
        let decrypted: TestData = encryptor.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_different_passwords() {
        let encryptor1 = SessionEncryptor::new("password1").unwrap();
        let encryptor2 = SessionEncryptor::new("password2").unwrap();

        let data = TestData {
            message: "Test".to_string(),
            timestamp: 0,
        };

        let encrypted = encryptor1.encrypt(&data).unwrap();
        let result: Result<TestData, _> = encryptor2.decrypt(&encrypted);

        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_base64() {
        let encryptor = SessionEncryptor::new("test-password").unwrap();
        let result: Result<TestData, _> = encryptor.decrypt("invalid-base64!!!");

        assert!(result.is_err());
    }
}
