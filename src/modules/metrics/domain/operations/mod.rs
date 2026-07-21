/// Pure domain operation: Calculate cost for token usage
pub fn calculate_cost(input_tokens: u32, output_tokens: u32, model: &str) -> f64 {
    // Pricing per 1M tokens (approximate)
    let pricing = match model {
        m if m.contains("gpt-4") => (15.0, 60.0), // GPT-4
        m if m.contains("gpt-3.5") => (0.5, 1.5), // GPT-3.5
        m if m.contains("claude-3-opus") => (15.0, 75.0),
        m if m.contains("claude-3-sonnet") => (3.0, 15.0),
        m if m.contains("claude-3-haiku") => (0.25, 1.25),
        m if m.contains("gemini-pro") => (1.25, 5.0),
        m if m.contains("gemini-ultra") => (7.0, 15.0),
        _ => (1.0, 5.0), // Default pricing
    };

    let (input_per_m, output_per_m) = pricing;
    let input_cost = (input_tokens as f64 / 1_000_000.0) * input_per_m;
    let output_cost = (output_tokens as f64 / 1_000_000.0) * output_per_m;

    input_cost + output_cost
}

/// Pure domain operation: Calculate throughput
pub fn calculate_throughput(tokens: u32, duration_ms: u64) -> f64 {
    if duration_ms == 0 {
        return 0.0;
    }
    (tokens as f64 / duration_ms as f64) * 1000.0
}

/// Pure domain operation: Determine if latency is acceptable
pub fn is_latency_acceptable(latency_ms: f64, model: &str) -> bool {
    let threshold = match model {
        m if m.contains("gpt-4") => 5000.0,
        m if m.contains("claude-3") => 4000.0,
        m if m.contains("gemini") => 3000.0,
        _ => 5000.0,
    };
    latency_ms <= threshold
}

/// Pure domain operation: Calculate error rate
pub fn calculate_error_rate(error_count: u64, total_count: u64) -> f64 {
    if total_count == 0 {
        return 0.0;
    }
    (error_count as f64 / total_count as f64) * 100.0
}
