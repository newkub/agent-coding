use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;

/// Cache key for AI responses
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CacheKey {
    pub model: String,
    pub prompt_hash: u64,
    pub temperature: u32, // Convert f32 to u32 for Hash/Eq
}

/// Cached AI response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedResponse {
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Response cache for AI requests
pub struct ResponseCache {
    cache: Arc<Cache<CacheKey, CachedResponse>>,
}

impl ResponseCache {
    pub(crate) fn new(ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(
                Cache::builder()
                    .time_to_live(Duration::from_secs(ttl_seconds))
                    .max_capacity(1000)
                    .build(),
            ),
        }
    }

    pub(crate) async fn get(&self, key: &CacheKey) -> Option<CachedResponse> {
        self.cache.get(key).await
    }

    pub(crate) async fn put(&self, key: CacheKey, response: CachedResponse) {
        self.cache.insert(key, response).await;
    }

    pub(crate) async fn invalidate(&self, key: &CacheKey) {
        self.cache.invalidate(key).await;
    }

    pub(crate) async fn clear(&self) {
        self.cache.invalidate_all();
    }
}

impl Default for ResponseCache {
    fn default() -> Self {
        Self::new(3600)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_hit() {
        let cache = ResponseCache::new(3600);
        let key = CacheKey {
            model: "gpt-4".to_string(),
            prompt_hash: 123,
            temperature: 70, // 0.7 * 100
        };
        let response = CachedResponse {
            content: "test response".to_string(),
            timestamp: chrono::Utc::now(),
        };
        
        cache.put(key.clone(), response.clone()).await;
        let cached = cache.get(&key).await;
        
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().content, "test response");
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let cache = ResponseCache::new(3600);
        let key = CacheKey {
            model: "gpt-4".to_string(),
            prompt_hash: 123,
            temperature: 70,
        };
        
        let cached = cache.get(&key).await;
        assert!(cached.is_none());
    }

    #[tokio::test]
    async fn test_cache_invalidate() {
        let cache = ResponseCache::new(3600);
        let key = CacheKey {
            model: "gpt-4".to_string(),
            prompt_hash: 123,
            temperature: 70,
        };
        let response = CachedResponse {
            content: "test response".to_string(),
            timestamp: chrono::Utc::now(),
        };
        
        cache.put(key.clone(), response).await;
        cache.invalidate(&key).await;
        let cached = cache.get(&key).await;
        
        assert!(cached.is_none());
    }
}
