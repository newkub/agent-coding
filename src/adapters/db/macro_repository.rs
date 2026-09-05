use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::modules::macros::domain::models::{Macro, MacroId};
use crate::modules::macros::ports::MacroRepository;
use crate::shared::kernel::result::AppResult;

/// In-memory macro repository for fast startup and CLI wiring.
///
/// Finished macros live in `macros`; `recording` tracks the ids of macros
/// whose recording is still in progress so `finish_recording` can tell an
/// active recording apart from an already-saved macro.
pub(crate) struct InMemoryMacroRepository {
    macros: Arc<RwLock<HashMap<String, Macro>>>,
    recording: Arc<RwLock<HashSet<String>>>,
}

impl InMemoryMacroRepository {
    pub(crate) fn new() -> Self {
        Self {
            macros: Arc::new(RwLock::new(HashMap::new())),
            recording: Arc::new(RwLock::new(HashSet::new())),
        }
    }
}

impl Default for InMemoryMacroRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl MacroRepository for InMemoryMacroRepository {
    async fn save(&self, macro_def: &Macro) -> AppResult<()> {
        let mut macros = self.macros.write().await;
        macros.insert(macro_def.id.as_str().to_string(), macro_def.clone());
        Ok(())
    }

    async fn save_recording(&self, macro_def: &Macro) -> AppResult<()> {
        let mut macros = self.macros.write().await;
        macros.insert(macro_def.id.as_str().to_string(), macro_def.clone());
        let mut recording = self.recording.write().await;
        recording.insert(macro_def.id.as_str().to_string());
        Ok(())
    }

    async fn finish_recording(&self, id: &MacroId) -> AppResult<Option<Macro>> {
        let mut recording = self.recording.write().await;
        if !recording.remove(id.as_str()) {
            return Ok(None);
        }
        let macros = self.macros.read().await;
        Ok(macros.get(id.as_str()).cloned())
    }

    async fn find_by_id(&self, id: &MacroId) -> AppResult<Option<Macro>> {
        let macros = self.macros.read().await;
        Ok(macros.get(id.as_str()).cloned())
    }

    async fn find_all(&self) -> AppResult<Vec<Macro>> {
        let macros = self.macros.read().await;
        let mut all: Vec<Macro> = macros.values().cloned().collect();
        all.sort_by_key(|m| m.created_at);
        Ok(all)
    }

    async fn delete(&self, id: &MacroId) -> AppResult<()> {
        let mut macros = self.macros.write().await;
        macros.remove(id.as_str());
        let mut recording = self.recording.write().await;
        recording.remove(id.as_str());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn sample_macro(id: &str, name: &str) -> Macro {
        Macro::create(
            MacroId::from_string(id.to_string()),
            name.to_string(),
            String::new(),
            Utc::now(),
        )
    }

    #[tokio::test]
    async fn test_save_and_find() {
        let repo = InMemoryMacroRepository::new();
        repo.save(&sample_macro("m1", "build")).await.unwrap();

        let found = repo
            .find_by_id(&MacroId::from_string("m1".to_string()))
            .await
            .unwrap();
        assert_eq!(found.unwrap().name, "build");
        assert_eq!(repo.find_all().await.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_recording_lifecycle() {
        let repo = InMemoryMacroRepository::new();
        let macro_def = sample_macro("m1", "rec");
        repo.save_recording(&macro_def).await.unwrap();

        let finished = repo.finish_recording(&macro_def.id).await.unwrap();
        assert!(finished.is_some());
        // A second finish returns None - the recording is no longer active.
        assert!(repo
            .finish_recording(&macro_def.id)
            .await
            .unwrap()
            .is_none());
    }

    #[tokio::test]
    async fn test_delete() {
        let repo = InMemoryMacroRepository::new();
        let macro_def = sample_macro("m1", "rec");
        repo.save_recording(&macro_def).await.unwrap();
        repo.delete(&macro_def.id).await.unwrap();

        assert!(repo.find_by_id(&macro_def.id).await.unwrap().is_none());
        assert!(repo
            .finish_recording(&macro_def.id)
            .await
            .unwrap()
            .is_none());
    }
}
