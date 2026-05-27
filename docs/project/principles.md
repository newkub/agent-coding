---
title: Principles
description: Principles และ best practices ที่ใช้
---

# Principles

## Architecture Principles

### Domain Purity

**Rule**: Domain layer ต้องมีเฉพาะ pure functions - ไม่มี IO, ไม่มี state mutation, ไม่มี external dependencies

**Implementation**:
- Domain functions ต้อง pure: เดียวกัน input = เดียวกัน output
- ไม่มี side effects ใน domain layer
- ไม่มี external calls ใน domain layer
- ไม่มี state mutation ใน domain layer

**Example**:
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

### Separation of Concerns

**Rule**: แต่ละ layer มี single responsibility: Domain (business logic), Application (orchestration), Adapters (side effects), Presentation (entry points)

**Implementation**:
- Domain: Business logic เท่านั้น
- Application: Orchestration และ workflows
- Adapters: Side effects เท่านั้น
- Presentation: Entry points เท่านั้น

**Example**:
```rust
// Domain - Business logic
fn validate_message(message: &Message) -> Result<(), ValidationError> {
    // Pure validation logic
}

// Application - Orchestration
async fn process_message(message: Message) -> Result<Response> {
    // Orchestrate validation, AI call, storage
}

// Adapter - Side effect
async fn save_to_db(message: Message) -> Result<()> {
    // Database IO
}

// Presentation - Entry point
async fn handle_command(cmd: Command) {
    // CLI/TUI initialization
}
```

### Dependency Inversion

**Rule**: Depend on abstractions (traits), not concretions - define ports in domain, implement adapters

**Implementation**:
- Define traits ใน domain layer
- Implement traits ใน adapters layer
- Inject dependencies ผ่าน traits
- Easy to test ด้วย mocks

**Example**:
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

### Immutability

**Rule**: Prefer immutable data structures และ pure transformations มากกว่า mutable state

**Implementation**:
- ใช้ `&T` มากกว่า `&mut T`
- ใช้ functional patterns (map, filter, fold)
- Avoid interior mutability ถ้าไม่จำเป็น
- Clone แทน mutation ถ้าเล็ก

**Example**:
```rust
// ✅ Good - Immutable
fn process_messages(messages: &[Message]) -> Vec<Response> {
    messages.iter().map(|m| transform(m)).collect()
}

// ❌ Bad - Mutable
fn process_messages(messages: &mut Vec<Message>) -> Vec<Response> {
    messages.iter_mut().for_each(|m| *m = transform(m));
    // ...
}
```

### Composition over Inheritance

**Rule**: Compose small, focused functions และ modules มากกว่า deep inheritance hierarchies

**Implementation**:
- Small, focused functions
- Compose functions ด้วย pipe operators
- Use traits สำหรับ shared behavior
- Avoid deep inheritance

**Example**:
```rust
// ✅ Good - Composition
fn process(input: Input) -> Output {
    input
        .validate()
        .transform()
        .save()
}

// ❌ Bad - Inheritance
trait Processor {
    fn validate(&self) -> Self;
    fn transform(&self) -> Self;
    fn save(&self) -> Self;
}
```

## Code Quality Principles

### Type Safety

**Rule**: Leverage Rust's type system สำหรับ compile-time guarantees

**Implementation**:
- ใช้ `Result<T, E>` สำหรับ error handling
- ใช้ `Option<T>` สำหรับ optional values
- ใช้ enums สำหรับ state machines
- Avoid `unwrap()` ใน production code

### Error Handling

**Rule**: Comprehensive error types พร้อม clear messages

**Implementation**:
- Define custom error types
- Use `thiserror` สำหรับ error derivation
- Provide context กับ errors
- Handle errors อย่างเหมาะสม

**Example**:
```rust
#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("AI provider error: {0}")]
    AIProvider(#[from] AIError),
    
    #[error("Database error: {0}")]
    Database(#[from] DbError),
    
    #[error("Validation error: {0}")]
    Validation(String),
}
```

### Testing

**Rule**: Write tests สำหรับ domain logic ก่อน, integration tests สำหรับ adapters

**Implementation**:
- Unit tests สำหรับ pure functions
- Integration tests สำหร adapters
- Mock external dependencies
- Test edge cases

### Documentation

**Rule**: Document public APIs และ complex logic

**Implementation**:
- Use `///` สำหรับ documentation comments
- Document function signatures
- Provide examples
- Explain design decisions

## Performance Principles

### Async/Await

**Rule**: Use async/await สำหรับ IO-bound operations

**Implementation**:
- Use `tokio` สำหร async runtime
- Async functions สำหรับ IO operations
- Avoid blocking in async context
- Use `tokio::spawn` สำหร parallel tasks

### Caching

**Rule**: Cache expensive operations สำหรับ performance

**Implementation**:
- Use `moka` สำหร async caching
- Cache AI responses
- Cache LSP results
- Invalidate cache เมื่อเหมาะสม

### Resource Management

**Rule**: Manage resources efficiently ด้วย RAII

**Implementation**:
- Use `Drop` สำหร cleanup
- Limit connection pools
- Use timeouts สำหร external calls
- Monitor resource usage

## Security Principles

### Input Validation

**Rule**: Validate all inputs ก่อน processing

**Implementation**:
- Validate file paths
- Sanitize shell commands
- Validate API inputs
- Use types สำหร validation

### Least Privilege

**Rule**: Grant minimum permissions ที่จำเป็น

**Implementation**:
- Restrict file access
- Limit command execution
- Scope API keys
- Audit access logs

### Secrets Management

**Rule**: Never hardcode secrets ใน code

**Implementation**:
- Use environment variables
- Use config files
- Encrypt secrets at rest
- Rotate keys regularly

## User Experience Principles

### Clarity

**Rule**: Provide clear error messages แล feedback

**Implementation**:
- User-friendly error messages
- Progress indicators
- Clear status updates
- Helpful suggestions

### Consistency

**Rule**: Maintain consistent UI แล behavior

**Implementation**:
- Consistent keyboard shortcuts
- Consistent terminology
- Consistent layout
- Consistent behavior

### Performance

**Rule**: Provide responsive UI แม้กับ heavy operations

**Implementation**:
- Non-blocking operations
- Streaming responses
- Progress updates
- Cancellation support
