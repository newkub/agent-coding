use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// A simple in-memory token bucket rate limiter.
///
/// `rate` is the number of tokens added per second; `burst` is the maximum
/// number of tokens that can be consumed at once. Calls to `acquire` wait
/// asynchronously until a token is available.
#[derive(Clone)]
pub(crate) struct RateLimiter {
    rate: f64,
    burst: f64,
    state: Arc<Mutex<RateLimiterState>>,
}

struct RateLimiterState {
    tokens: f64,
    last_update: Instant,
}

impl RateLimiter {
    /// Create a new rate limiter with `rate` tokens/second and `burst` capacity.
    pub(crate) fn new(rate: f64, burst: f64) -> Self {
        Self {
            rate,
            burst,
            state: Arc::new(Mutex::new(RateLimiterState {
                tokens: burst,
                last_update: Instant::now(),
            })),
        }
    }

    /// Acquire one token, waiting if the bucket is empty.
    pub(crate) async fn acquire(&self) {
        loop {
            let mut state = self.state.lock().await;
            let now = Instant::now();
            let elapsed = now.duration_since(state.last_update).as_secs_f64();
            state.tokens = (state.tokens + elapsed * self.rate).min(self.burst);
            state.last_update = now;

            if state.tokens >= 1.0 {
                state.tokens -= 1.0;
                return;
            }

            let needed = 1.0 - state.tokens;
            let wait = Duration::from_secs_f64(needed / self.rate);
            drop(state);
            tokio::time::sleep(wait).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_allows_burst_then_throttles() {
        let limiter = RateLimiter::new(10.0, 2.0);
        let start = Instant::now();
        limiter.acquire().await;
        limiter.acquire().await;
        assert!(start.elapsed() < Duration::from_millis(10));

        // The third call must wait for at least one token to be replenished.
        let start = Instant::now();
        limiter.acquire().await;
        assert!(start.elapsed() >= Duration::from_millis(80));
    }
}
