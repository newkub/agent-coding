---
title: Clean Architecture
description: Clean Architecture (FP-style) implementation guide
---

# Clean Architecture (FP-style)

## Overview

agent-tui ใช้ Clean Architecture แบบ Functional Programming (FP-style) ซึ่งแตกต่างจาก Clean Architecture แบบดั้งเดิมที่ใช้ OOP

## Core Principles

### 1. Domain Purity

**Rule**: Domain layer ต้องมีเฉพาะ pure functions

```rust
// ✅ Good - Pure function
fn calculate_score(messages: &[Message]) -> u32 {
    messages.iter().map(|m| m.tokens).sum()
}

// ❌ Bad - Has side effect
fn calculate_score(messages: &[Message]) -> u32 {
    println!("Calculating..."); // Side effect
    messages.iter().map(|m| m.tokens).sum()
}
```

### 2. Separation of Concerns

แต่ละ layer มี single responsibility:

- **Domain**: Business logic เท่านั้น
- **Application**: Orchestration และ workflows
- **Adapters**: Side effects เท่านั้น
- **Presentation**: Entry points เท่านั้น

### 3. Dependency Inversion

Depend on abstractions (traits), not concretions:

```rust
// Domain - Port (trait)
trait AIProvider {
    async fn generate(&self, prompt: &str) -> Result<String>;
}

// Adapter - Implementation
struct OpenAIProvider {
    api_key: String,
}

impl AIProvider for OpenAIProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        // OpenAI API call
    }
}
```

## Layer Structure

### Domain Layer

**Location**: `src/modules/*/domain/`

**Responsibilities**:
- Business logic
- Domain models
- Domain operations
- Validators
- Events

**Characteristics**:
- 100% pure functions
- No IO
- No state mutation
- No external dependencies

**Example**:
```rust
// src/modules/session/domain/operations/create_session.rs
pub fn create_session(name: String) -> Result<Session, ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::EmptyName);
    }
    
    Ok(Session {
        id: Uuid::new_v4(),
        name,
        created_at: Utc::now(),
        messages: Vec::new(),
    })
}
```

### Application Layer

**Location**: `src/modules/*/application/`

**Responsibilities**:
- Orchestration
- Workflows
- Use cases
- Services

**Characteristics**:
- Pipeline-style composition
- Async operations
- Error handling
- Transaction management

**Example**:
```rust
// src/modules/session/application/usecases/create_session_usecase.rs
pub struct CreateSessionUseCase {
    session_repository: Arc<dyn SessionRepository>,
}

impl CreateSessionUseCase {
    pub async fn execute(&self, name: String) -> Result<Session> {
        // Domain operation (pure)
        let session = domain::create_session(name)?;
        
        // Adapter operation (side effect)
        self.session_repository.save(session.clone()).await?;
        
        Ok(session)
    }
}
```

### Adapters Layer

**Location**: `src/adapters/`

**Responsibilities**:
- Database operations
- HTTP clients
- File operations
- External services
- UI rendering

**Characteristics**:
- Side effects เท่านั้น
- Implement domain ports
- State management
- IO operations

**Example**:
```rust
// src/adapters/db/session_repository.rs
pub struct SqliteSessionRepository {
    pool: SqlitePool,
}

impl SessionRepository for SqliteSessionRepository {
    async fn save(&self, session: Session) -> Result<()> {
        sqlx::query(
            "INSERT INTO sessions (id, name, created_at) VALUES (?, ?, ?)"
        )
        .bind(session.id)
        .bind(session.name)
        .bind(session.created_at)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}
```

### Presentation Layer

**Location**: `src/presentation/`

**Responsibilities**:
- CLI initialization
- TUI initialization
- DI container
- Entry points

**Characteristics**:
- Thin layer
- Composition root
- Dependency injection
- No business logic

**Example**:
```rust
// src/presentation/di.rs
pub struct DIContainer {
    session_repository: Arc<dyn SessionRepository>,
    ai_provider: Arc<dyn AIProvider>,
}

impl DIContainer {
    pub fn new() -> Self {
        let pool = SqlitePool::connect("sqlite://sessions.db").await.unwrap();
        let session_repository = Arc::new(SqliteSessionRepository::new(pool));
        let ai_provider = Arc::new(OpenAIProvider::new());
        
        Self {
            session_repository,
            ai_provider,
        }
    }
    
    pub fn create_session_use_case(&self) -> CreateSessionUseCase {
        CreateSessionUseCase {
            session_repository: self.session_repository.clone(),
        }
    }
}
```

## Dependency Flow

```
Presentation → Application → Domain → Ports
                 ↓            ↓
               Adapters ←─── Implementations
```

**Rules**:
- Presentation depends on Application
- Application depends on Domain
- Domain defines Ports (traits)
- Adapters implement Ports
- No circular dependencies

## Module Organization

### Vertical Slice Architecture

แต่ละ module เป็น vertical slice:

```
src/modules/
├── agent/
│   ├── domain/
│   │   ├── models/
│   │   ├── operations/
│   │   └── validators/
│   ├── application/
│   │   ├── usecases/
│   │   └── services/
│   └── ports/
├── session/
│   ├── domain/
│   ├── application/
│   └── ports/
└── message/
    ├── domain/
    ├── application/
    └── ports/
```

**Benefits**:
- Independent modules
- Clear boundaries
- Easy to test
- Easy to maintain

## Testing Strategy

### Domain Layer Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_session_valid_name() {
        let session = create_session("test".to_string()).unwrap();
        assert_eq!(session.name, "test");
    }
    
    #[test]
    fn test_create_session_empty_name() {
        let result = create_session("".to_string());
        assert!(result.is_err());
    }
}
```

### Application Layer Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    
    mock! {
        SessionRepository {}
        
        impl SessionRepository for SessionRepository {
            async fn save(&self, session: Session) -> Result<()>;
        }
    }
    
    #[tokio::test]
    async fn test_create_session_usecase() {
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
}
```

## Common Patterns

### Error Handling

```rust
// Domain - Define error types
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Name cannot be empty")]
    EmptyName,
    #[error("Name too long")]
    NameTooLong,
}

// Application - Convert to app errors
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),
    #[error("Database error: {0}")]
    Database(#[from] DbError),
}
```

### Pipeline Composition

```rust
pub async fn process_message(
    message: Message,
    validator: Arc<dyn Validator>,
    ai_provider: Arc<dyn AIProvider>,
    repository: Arc<dyn Repository>,
) -> Result<Response> {
    // Validate (pure)
    validator.validate(&message)?;
    
    // Generate (IO)
    let response = ai_provider.generate(&message.content).await?;
    
    // Save (IO)
    repository.save(response.clone()).await?;
    
    Ok(response)
}
```

## Best Practices

1. **Keep Domain Pure**: No side effects in domain layer
2. **Use Traits for Abstractions**: Define ports in domain
3. **Inject Dependencies**: Use DI container in presentation
4. **Test Domain First**: Unit tests for pure functions
5. **Mock Adapters**: Use mocks for adapter testing
6. **Async in Application**: Use async/await for IO operations
7. **Error Propagation**: Use Result types throughout
8. **Immutable Data**: Prefer immutable data structures

## Migration Guide

### From OOP to FP

**OOP Style**:
```rust
struct SessionService {
    repository: Arc<dyn Repository>,
}

impl SessionService {
    fn create(&self, name: String) -> Result<Session> {
        let session = Session::new(name);
        self.repository.save(session)?;
        Ok(session)
    }
}
```

**FP Style**:
```rust
// Domain (pure)
fn create_session(name: String) -> Result<Session, ValidationError> {
    // Pure logic
}

// Application (orchestration)
pub struct CreateSessionUseCase {
    repository: Arc<dyn Repository>,
}

impl CreateSessionUseCase {
    pub async fn execute(&self, name: String) -> Result<Session> {
        let session = create_session(name)?;
        self.repository.save(session).await?;
        Ok(session)
    }
}
```
