use crate::modules::metrics::domain::models::{MetricsSummary, PerformanceMetric, TokenUsage};
use crate::modules::metrics::ports::{MetricsRepository, TokenUsageRepository};
use crate::shared::kernel::result::AppResult;

/// Use case: Record token usage
pub(crate) async fn record_token_usage<R>(
    repo: &R,
    session_id: String,
    model: String,
    provider: String,
    input_tokens: u32,
    output_tokens: u32,
) -> AppResult<TokenUsage>
where
    R: TokenUsageRepository,
{
    let cost = crate::modules::metrics::domain::operations::calculate_cost(
        input_tokens,
        output_tokens,
        &model,
    );

    let usage = TokenUsage::new(
        session_id,
        model,
        provider,
        input_tokens,
        output_tokens,
        cost,
    );

    repo.save(&usage).await?;
    Ok(usage)
}

/// Use case: Record performance metric
pub(crate) async fn record_metric<R>(repo: &R, metric: PerformanceMetric) -> AppResult<()>
where
    R: MetricsRepository,
{
    repo.save_metric(&metric).await
}

/// Use case: Get metrics summary
pub(crate) async fn get_summary<R, T>(
    token_repo: &T,
    metrics_repo: &R,
    session_id: Option<&str>,
) -> AppResult<MetricsSummary>
where
    R: MetricsRepository,
    T: TokenUsageRepository,
{
    let usages = if let Some(sid) = session_id {
        token_repo.find_by_session(sid).await?
    } else {
        token_repo.find_all().await?
    };

    let metrics = metrics_repo.get_all().await?;

    Ok(MetricsSummary::calculate(&usages, &metrics))
}

/// Use case: Get cost breakdown by model
pub(crate) async fn get_cost_by_model<R>(
    repo: &R,
) -> AppResult<std::collections::HashMap<String, CostBreakdown>>
where
    R: TokenUsageRepository,
{
    let usages = repo.find_all().await?;

    let mut breakdown: std::collections::HashMap<String, CostBreakdown> =
        std::collections::HashMap::new();

    for usage in usages {
        let entry = breakdown
            .entry(usage.model.clone())
            .or_insert(CostBreakdown {
                total_tokens: 0,
                total_cost: 0.0,
                request_count: 0,
            });

        entry.total_tokens += usage.total_tokens as u64;
        entry.total_cost += usage.cost_usd;
        entry.request_count += 1;
    }

    Ok(breakdown)
}

#[derive(Debug, Clone)]
pub(crate) struct CostBreakdown {
    pub total_tokens: u64,
    pub total_cost: f64,
    pub request_count: u64,
}
