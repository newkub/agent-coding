// CLI handlers - one module per command family
// Each handler is a thin imperative shell that:
//   1. Builds the DI container with concrete adapters
//   2. Calls the appropriate use case from `application/usecases/`
//   3. Renders the result via `output` helpers

pub(crate) mod audit;
pub(crate) mod automation;
pub(crate) mod guardrail;
pub(crate) mod headless;
pub(crate) mod onboarding;
pub(crate) mod performance;
pub(crate) mod session;
pub(crate) mod share;
pub(crate) mod subagent;
pub(crate) mod tui;
pub(crate) mod version;
