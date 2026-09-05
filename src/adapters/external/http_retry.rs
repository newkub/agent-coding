use std::time::Duration;

use reqwest::{RequestBuilder, Response, StatusCode};
use tracing::{info_span, warn};

use crate::shared::kernel::result::AppError;

const MAX_ATTEMPTS: usize = 3;

/// Send a request with bounded exponential backoff for transient failures.
///
/// The caller supplies a fresh `RequestBuilder` for each attempt because
/// reqwest request bodies are not guaranteed to be replayable.
pub(crate) async fn send_with_retry<F>(mut request: F) -> Result<Response, AppError>
where
    F: FnMut() -> RequestBuilder,
{
    let correlation_id = uuid::Uuid::new_v4().to_string();
    let span = info_span!("http_request", correlation_id = %correlation_id);

    let mut delay = Duration::from_millis(250);
    let mut last_error = None;

    for attempt in 1..=MAX_ATTEMPTS {
        let builder = request().header("x-correlation-id", &correlation_id);
        match builder.send().await {
            Ok(response) if should_retry_status(response.status()) && attempt < MAX_ATTEMPTS => {
                warn!(
                    parent: &span,
                    attempt,
                    status = %response.status(),
                    delay_ms = delay.as_millis() as u64,
                    "retryable HTTP status; retrying request"
                );
                last_error = Some(AppError::State(format!(
                    "HTTP request returned retryable status {}",
                    response.status()
                )));
                tokio::time::sleep(delay).await;
                delay *= 2;
            }
            Ok(response) => return Ok(response),
            Err(error) if is_retryable_error(&error) && attempt < MAX_ATTEMPTS => {
                warn!(
                    parent: &span,
                    attempt,
                    error = %error,
                    delay_ms = delay.as_millis() as u64,
                    "transient HTTP error; retrying request"
                );
                last_error = Some(AppError::State(format!("transient HTTP error: {error}")));
                tokio::time::sleep(delay).await;
                delay *= 2;
            }
            Err(error) => {
                return Err(AppError::State(format!(
                    "HTTP request failed (correlation_id={correlation_id}): {error}"
                )));
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        AppError::State(format!(
            "HTTP request failed (correlation_id={correlation_id})"
        ))
    }))
}

fn should_retry_status(status: StatusCode) -> bool {
    status == StatusCode::TOO_MANY_REQUESTS
        || status == StatusCode::REQUEST_TIMEOUT
        || status.is_server_error()
}

fn is_retryable_error(error: &reqwest::Error) -> bool {
    error.is_timeout() || error.is_connect() || error.is_request()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retryable_statuses() {
        assert!(should_retry_status(StatusCode::TOO_MANY_REQUESTS));
        assert!(should_retry_status(StatusCode::REQUEST_TIMEOUT));
        assert!(should_retry_status(StatusCode::BAD_GATEWAY));
        assert!(!should_retry_status(StatusCode::BAD_REQUEST));
        assert!(!should_retry_status(StatusCode::OK));
    }
}
