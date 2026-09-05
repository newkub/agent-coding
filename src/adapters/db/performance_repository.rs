use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::adapters::db::db_err;
use crate::modules::performance::domain::models::metrics::{
    OptimizationSuggestion, PerformanceComparison, PerformanceMetrics, PerformanceSnapshot,
};
use crate::modules::performance::domain::operations::performance_operations::{
    analyze_performance, sort_suggestions_by_priority,
};
use crate::modules::performance::ports::{OptimizationManager, SnapshotManager};
use crate::shared::kernel::result::AppError;

/// SQLite-backed performance snapshot manager.
pub(crate) struct SqliteSnapshotManager {
    pool: SqlitePool,
}

impl SqliteSnapshotManager {
    pub(crate) const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn snapshot_from_row(row: &sqlx::sqlite::SqliteRow) -> Result<PerformanceSnapshot, AppError> {
        let data: String = row.get("data");
        serde_json::from_str(&data).map_err(|e| {
            AppError::Database(format!("deserialize performance snapshot failed: {e}"))
        })
    }
}

#[async_trait]
impl SnapshotManager for SqliteSnapshotManager {
    async fn create_snapshot(
        &self,
        name: String,
        metrics: PerformanceMetrics,
    ) -> Result<PerformanceSnapshot, AppError> {
        let snapshot = PerformanceSnapshot::new(name, metrics);
        let metrics_json = serde_json::to_string(&snapshot.metrics).map_err(|e| {
            AppError::Database(format!("serialize performance metrics failed: {e}"))
        })?;
        let baseline_json = snapshot
            .baseline
            .as_ref()
            .map(serde_json::to_string)
            .transpose()
            .map_err(|e| {
                AppError::Database(format!("serialize performance baseline failed: {e}"))
            })?;
        let data = serde_json::to_string(&snapshot).map_err(|e| {
            AppError::Database(format!("serialize performance snapshot failed: {e}"))
        })?;

        sqlx::query(
            r#"
            INSERT INTO performance_snapshots
                (id, name, created_at, metrics, baseline, data)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&snapshot.id)
        .bind(&snapshot.name)
        .bind(snapshot.created_at.to_rfc3339())
        .bind(metrics_json)
        .bind(baseline_json)
        .bind(data)
        .execute(&self.pool)
        .await
        .map_err(|e| db_err("insert performance snapshot", e))?;

        Ok(snapshot)
    }

    async fn get_snapshot(&self, id: &str) -> Result<PerformanceSnapshot, AppError> {
        let row = sqlx::query("SELECT data FROM performance_snapshots WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| db_err("load performance snapshot", e))?
            .ok_or_else(|| AppError::NotFound(format!("Snapshot {id} not found")))?;

        Self::snapshot_from_row(&row)
    }

    async fn list_snapshots(&self) -> Result<Vec<PerformanceSnapshot>, AppError> {
        let rows = sqlx::query("SELECT data FROM performance_snapshots ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| db_err("list performance snapshots", e))?;

        rows.iter().map(Self::snapshot_from_row).collect()
    }

    async fn delete_snapshot(&self, id: &str) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM performance_snapshots WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("delete performance snapshot", e))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Snapshot {id} not found")));
        }
        Ok(())
    }

    async fn compare_snapshots(
        &self,
        id1: &str,
        id2: &str,
    ) -> Result<PerformanceComparison, AppError> {
        let snapshot1 = self.get_snapshot(id1).await?;
        let snapshot2 = self.get_snapshot(id2).await?;

        Ok(PerformanceComparison {
            cpu_diff: snapshot2.metrics.cpu_usage - snapshot1.metrics.cpu_usage,
            memory_diff: snapshot2.metrics.memory_usage as i64
                - snapshot1.metrics.memory_usage as i64,
            response_time_diff: snapshot2
                .metrics
                .response_time_ms
                .zip(snapshot1.metrics.response_time_ms)
                .map(|(a, b)| a as i64 - b as i64),
            throughput_diff: snapshot2
                .metrics
                .throughput
                .zip(snapshot1.metrics.throughput)
                .map(|(a, b)| a - b),
            error_rate_diff: snapshot2
                .metrics
                .error_rate
                .zip(snapshot1.metrics.error_rate)
                .map(|(a, b)| a - b),
        })
    }
}

/// SQLite-backed optimization suggestion manager.
pub(crate) struct SqliteOptimizationManager {
    pool: SqlitePool,
}

impl SqliteOptimizationManager {
    pub(crate) const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn suggestion_from_row(
        row: &sqlx::sqlite::SqliteRow,
    ) -> Result<OptimizationSuggestion, AppError> {
        let data: String = row.get("data");
        serde_json::from_str(&data).map_err(|e| {
            AppError::Database(format!("deserialize optimization suggestion failed: {e}"))
        })
    }

    async fn get_suggestion(
        &self,
        suggestion_id: &str,
    ) -> Result<OptimizationSuggestion, AppError> {
        let row = sqlx::query("SELECT data FROM optimization_suggestions WHERE id = ?")
            .bind(suggestion_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| db_err("load optimization suggestion", e))?
            .ok_or_else(|| AppError::NotFound(format!("Suggestion {suggestion_id} not found")))?;
        Self::suggestion_from_row(&row)
    }
}

#[async_trait]
impl OptimizationManager for SqliteOptimizationManager {
    async fn generate_suggestions(
        &self,
        metrics: &PerformanceMetrics,
    ) -> Result<Vec<OptimizationSuggestion>, AppError> {
        let mut suggestions = analyze_performance(metrics);
        sort_suggestions_by_priority(&mut suggestions);

        // Replace unapplied suggestions with the latest analysis while keeping
        // applied suggestions as historical records.
        sqlx::query("DELETE FROM optimization_suggestions WHERE applied_at IS NULL")
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("clear unapplied optimization suggestions", e))?;

        for suggestion in &suggestions {
            let data = serde_json::to_string(suggestion).map_err(|e| {
                AppError::Database(format!("serialize optimization suggestion failed: {e}"))
            })?;
            sqlx::query(
                r#"
                INSERT INTO optimization_suggestions
                    (id, category, title, impact, effort, estimated_improvement,
                     created_at, applied_at, data)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&suggestion.id)
            .bind(format!("{:?}", suggestion.category))
            .bind(&suggestion.title)
            .bind(format!("{:?}", suggestion.impact))
            .bind(format!("{:?}", suggestion.effort))
            .bind(suggestion.estimated_improvement)
            .bind(suggestion.created_at.to_rfc3339())
            .bind(suggestion.applied_at.map(|d| d.to_rfc3339()))
            .bind(data)
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("insert optimization suggestion", e))?;
        }

        Ok(suggestions)
    }

    async fn apply_suggestion(&self, suggestion_id: &str) -> Result<(), AppError> {
        let mut suggestion = self.get_suggestion(suggestion_id).await?;
        if suggestion.applied_at.is_none() {
            suggestion.mark_applied();
            let data = serde_json::to_string(&suggestion).map_err(|e| {
                AppError::Database(format!("serialize optimization suggestion failed: {e}"))
            })?;
            sqlx::query(
                "UPDATE optimization_suggestions SET applied_at = ?, data = ? WHERE id = ?",
            )
            .bind(suggestion.applied_at.map(|d| d.to_rfc3339()))
            .bind(data)
            .bind(suggestion_id)
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("apply optimization suggestion", e))?;
        }
        Ok(())
    }

    async fn list_suggestions(&self) -> Result<Vec<OptimizationSuggestion>, AppError> {
        let rows =
            sqlx::query("SELECT data FROM optimization_suggestions ORDER BY created_at DESC")
                .fetch_all(&self.pool)
                .await
                .map_err(|e| db_err("list optimization suggestions", e))?;

        rows.iter().map(Self::suggestion_from_row).collect()
    }

    async fn dismiss_suggestion(&self, suggestion_id: &str) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM optimization_suggestions WHERE id = ?")
            .bind(suggestion_id)
            .execute(&self.pool)
            .await
            .map_err(|e| db_err("dismiss optimization suggestion", e))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!(
                "Suggestion {suggestion_id} not found"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::db::test_pool;
    use crate::modules::performance::domain::models::metrics::PerformanceMetrics;

    #[tokio::test]
    async fn test_snapshot_round_trip() {
        let pool = test_pool().await;
        let manager = SqliteSnapshotManager::new(pool);
        let snapshot = manager
            .create_snapshot("baseline".to_string(), PerformanceMetrics::new())
            .await
            .unwrap();

        let loaded = manager.get_snapshot(&snapshot.id).await.unwrap();
        assert_eq!(loaded.name, "baseline");
    }

    #[tokio::test]
    async fn test_apply_suggestion_persists() {
        let pool = test_pool().await;
        let manager = SqliteOptimizationManager::new(pool);
        let mut metrics = PerformanceMetrics::new();
        metrics.cpu_usage = 95.0;
        let suggestions = manager.generate_suggestions(&metrics).await.unwrap();
        let suggestion = suggestions.first().unwrap();

        manager.apply_suggestion(&suggestion.id).await.unwrap();
        let suggestions = manager.list_suggestions().await.unwrap();
        assert!(suggestions
            .iter()
            .any(|s| s.id == suggestion.id && s.applied_at.is_some()));
    }
}
