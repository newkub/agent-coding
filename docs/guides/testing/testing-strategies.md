---
title: Testing Strategies
description: Testing strategies และ test patterns
---

# Testing Strategies

## Testing Pyramid

```
        E2E Tests
       /          \
      /            \
     /  Integration  \
    /      Tests      \
   /__________________\
  /   Unit Tests        \
 /______________________\
```

## Unit Tests

### Domain Layer Tests

**Purpose**: Test pure business logic

**Location**: `src/modules/*/domain/` (inline tests)

**Example**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_session_valid_name() {
        let session = create_session("test".to_string()).unwrap();
        assert_eq!(session.name, "test");
        assert!(!session.id.is_nil());
    }
    
    #[test]
    fn test_create_session_empty_name() {
        let result = create_session("".to_string());
        assert!(matches!(result, Err(ValidationError::EmptyName)));
    }
    
    #[test]
    fn test_create_session_name_too_long() {
        let long_name = "a".repeat(256);
        let result = create_session(long_name);
        assert!(matches!(result, Err(ValidationError::NameTooLong)));
    }
}
```

### Application Layer Tests

**Purpose**: Test use cases with mocked dependencies

**Location**: `tests/` or `src/modules/*/application/tests/`

**Example**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    
    mock! {
        SessionRepository {}
        
        impl SessionRepository for SessionRepository {
            async fn save(&self, session: Session) -> Result<()>;
            async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>>;
        }
    }
    
    #[tokio::test]
    async fn test_create_session_usecase_success() {
        let mut mock_repo = MockSessionRepository::new();
        mock_repo
            .expect_save()
            .returning(|_| Ok(()));
        
        let use_case = CreateSessionUseCase {
            session_repository: Arc::new(mock_repo),
        };
        
        let result = use_case.execute("test".to_string()).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_create_session_usecase_repository_error() {
        let mut mock_repo = MockSessionRepository::new();
        mock_repo
            .expect_save()
            .returning(|_| Err(DbError::ConnectionFailed));
        
        let use_case = CreateSessionUseCase {
            session_repository: Arc::new(mock_repo),
        };
        
        let result = use_case.execute("test".to_string()).await;
        assert!(result.is_err());
    }
}
```

## Integration Tests

### Database Integration Tests

**Purpose**: Test database operations

**Location**: `tests/integration/`

**Example**:
```rust
#[tokio::test]
async fn test_session_repository_integration() {
    // Setup in-memory database
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    
    // Create repository
    let repository = SqliteSessionRepository::new(pool);
    
    // Test save
    let session = Session {
        id: Uuid::new_v4(),
        name: "test".to_string(),
        created_at: Utc::now(),
        messages: Vec::new(),
    };
    
    repository.save(session.clone()).await.unwrap();
    
    // Test find
    let found = repository.find_by_id(session.id).await.unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().name, "test");
}
```

### API Integration Tests

**Purpose**: Test external API integration

**Location**: `tests/integration/`

**Example**:
```rust
#[tokio::test]
#[ignore] // Requires API key
async fn test_openai_provider_integration() {
    let provider = OpenAIProvider::new();
    
    let response = provider.generate("Say hello").await.unwrap();
    
    assert!(!response.is_empty());
}
```

## E2E Tests

### CLI E2E Tests

**Purpose**: Test complete CLI workflows

**Location**: `tests/e2e/`

**Example**:
```rust
#[tokio::test]
async fn test_cli_create_session_workflow() {
    let mut cmd = Command::cargo_bin("agent-tui")
        .unwrap()
        .arg("create-session")
        .arg("--name")
        .arg("test-session");
    
    cmd.assert().success();
}
```

## Test Utilities

### Test Fixtures

```rust
#[cfg(test)]
mod fixtures {
    use super::*;
    
    pub fn create_test_session() -> Session {
        Session {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            created_at: Utc::now(),
            messages: Vec::new(),
        }
    }
    
    pub fn create_test_message() -> Message {
        Message {
            id: Uuid::new_v4(),
            role: MessageRole::User,
            content: "test".to_string(),
            timestamp: Utc::now(),
            tokens: 10,
        }
    }
}
```

### Test Helpers

```rust
#[cfg(test)]
mod helpers {
    use super::*;
    
    pub async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();
        pool
    }
    
    pub async fn teardown_test_db(pool: SqlitePool) {
        pool.close().await;
    }
}
```

## Test Organization

### File Structure

```
tests/
├── unit/
│   ├── session_tests.rs
│   ├── message_tests.rs
│   └── agent_tests.rs
├── integration/
│   ├── db_tests.rs
│   ├── api_tests.rs
│   └── git_tests.rs
└── e2e/
    ├── cli_tests.rs
    └── tui_tests.rs
```

### Running Specific Tests

```bash
# Run unit tests
cargo test --lib

# Run integration tests
cargo test --test '*'

# Run specific test file
cargo test --test session_tests

# Run specific test
cargo test test_create_session_valid_name

# Run tests with pattern
cargo test session
```

## Mocking

### Using Mockall

```rust
use mockall::mock;

mock! {
    AIProvider {}

    impl AIProvider for AIProvider {
        async fn generate(&self, prompt: &str) -> Result<String>;
    }
}

#[tokio::test]
async fn test_with_mock() {
    let mut mock_provider = MockAIProvider::new();
    mock_provider
        .expect_generate()
        .returning(|_| Ok("mock response".to_string()));
    
    let use_case = SendMessageUseCase {
        ai_provider: Arc::new(mock_provider),
    };
    
    let result = use_case.execute("test".to_string()).await;
    assert!(result.is_ok());
}
```

## Property-Based Testing

### Using Proptest

```bash
cargo add proptest
```

```rust
#[cfg(test)]
mod proptests {
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_session_name_not_empty(name in "[a-zA-Z0-9]{1,100}") {
            let result = create_session(name);
            assert!(result.is_ok());
        }
        
        #[test]
        fn test_session_name_empty(name in "") {
            let result = create_session(name);
            assert!(result.is_err());
        }
    }
}
```

## Test Coverage

### Using Tarpaulin

```bash
cargo install cargo-tarpaulin

# Generate HTML coverage
cargo tarpaulin --out Html

# Generate LCOV coverage
cargo tarpaulin --out Lcov

# Generate cobertura coverage
cargo tarpaulin --out Xml
```

### Coverage Goals

- **Domain Layer**: 100% (pure functions, easy to test)
- **Application Layer**: 90%+ (use cases with mocks)
- **Adapters Layer**: 80%+ (integration tests)
- **Presentation Layer**: 70%+ (E2E tests)

## Best Practices

1. **Test Domain First**: Unit tests for pure functions
2. **Mock External Dependencies**: Use mocks for adapters
3. **Test Error Paths**: Test both success and failure cases
4. **Use Descriptive Names**: Clear test names
5. **Arrange-Act-Assert**: Structure tests clearly
6. **Avoid Test Interdependence**: Tests should be independent
7. **Use Test Fixtures**: Reusable test data
8. **Keep Tests Fast**: Unit tests should be fast
9. **Run Tests Frequently**: Continuous testing
10. **Maintain Coverage**: Monitor coverage regularly
