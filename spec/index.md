# Test Specification

## Test Suites

- [Audit Domain Tests](./audit.spec.md)
- [Automation Domain Tests](./automation.spec.md)
- [Collaboration Domain Tests](./collaboration.spec.md)
- [Configuration Tests](./config.spec.md)
- [Diff Domain Tests](./diff.spec.md)
- [Guardrails Domain Tests](./guardrails.spec.md)
- [Headless Domain Tests](./headless.spec.md)
- [Macro Domain Tests](./macro.spec.md)
- [Metrics Domain Tests](./metrics.spec.md)
- [Onboarding Domain Tests](./onboarding.spec.md)
- [Performance Domain Tests](./performance.spec.md)
- [Sandbox Domain Tests](./sandbox.spec.md)
- [Session Domain Tests](./session.spec.md)
- [Share Domain Tests](./share.spec.md)
- [Snippet Domain Tests](./snippet.spec.md)
- [Subagents Domain Tests](./subagents.spec.md)
- [UI Domain Tests](./ui.spec.md)

## Summary

- **Total Test Suites**: 17
- **Total Test Cases**: 323
- **Coverage Target**: 100% for all domain modules
- **Test Framework**: Rust's built-in `#[test]` attribute
- **Test Location**: `tests/` directory

All tests follow AAA pattern (Arrange, Act, Assert) and cover:
- Happy paths
- Edge cases
- Error cases
- Serialization/deserialization
- Validation
- Business logic operations
