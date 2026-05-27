---
title: Agent Module API
description: Agent module APIs แล exports
---

# Agent Module API

## Overview

Agent module จัดการ AI agent configuration แล provider management

## Domain Models

### Agent

```rust
pub struct Agent {
    pub id: Uuid,
    pub name: String,
    pub provider: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub system_prompt: Option<String>,
}
```

### AgentConfig

```rust
pub struct AgentConfig {
    pub provider: String,
    pub model: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub system_prompt: Option<String>,
}
```

## Domain Operations

### Create Agent

```rust
pub fn create_agent(name: String, config: AgentConfig) -> Result<Agent, ValidationError>
```

**Parameters**:
- `name`: Agent name
- `config`: Agent configuration

**Returns**:
- `Ok(Agent)`: Created agent
- `Err(ValidationError)`: Validation error

**Example**:
```rust
let config = AgentConfig {
    provider: "openai".to_string(),
    model: "gpt-4".to_string(),
    temperature: 0.7,
    max_tokens: 4096,
    system_prompt: None,
};

let agent = create_agent("coding-assistant".to_string(), config)?;
```

## Application Use Cases

### CreateAgentUseCase

```rust
pub struct CreateAgentUseCase {
    agent_repository: Arc<dyn AgentRepository>,
}

impl CreateAgentUseCase {
    pub async fn execute(&self, name: String, config: AgentConfig) -> Result<Agent>
}
```

**Example**:
```rust
let use_case = container.create_agent_use_case();
let agent = use_case.execute("coding-assistant".to_string(), config).await?;
```

### GenerateResponseUseCase

```rust
pub struct GenerateResponseUseCase {
    ai_provider: Arc<dyn AIProvider>,
}

impl GenerateResponseUseCase {
    pub async fn execute(&self, prompt: String, config: AgentConfig) -> Result<String>
}
```

**Example**:
```rust
let use_case = container.generate_response_use_case();
let response = use_case.execute("Explain Rust ownership".to_string(), config).await?;
```

## Ports

### AIProvider

```rust
#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String>;
    async fn generate_with_config(&self, prompt: &str, config: &AgentConfig) -> Result<String>;
}
```

### AgentRepository

```rust
#[async_trait]
pub trait AgentRepository: Send + Sync {
    async fn save(&self, agent: Agent) -> Result<()>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Agent>>;
    async fn find_all(&self) -> Result<Vec<Agent>>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}
```

## Adapters

### OpenAIProvider

```rust
pub struct OpenAIProvider {
    api_key: String,
    client: reqwest::Client,
}

impl AIProvider for OpenAIProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        // OpenAI API implementation
    }
}
```

### AnthropicProvider

```rust
pub struct AnthropicProvider {
    api_key: String,
    client: reqwest::Client,
}

impl AIProvider for AnthropicProvider {
    async fn generate(&self, prompt: &str) -> Result<String> {
        // Anthropic API implementation
    }
}
```

## Usage Examples

### Create Agent

```rust
use agent_tui::modules::agent::application::usecases::CreateAgentUseCase;

let config = AgentConfig {
    provider: "openai".to_string(),
    model: "gpt-4".to_string(),
    temperature: 0.7,
    max_tokens: 4096,
    system_prompt: None,
};

let use_case = container.create_agent_use_case();
let agent = use_case.execute("coding-assistant".to_string(), config).await?;
```

### Generate Response

```rust
use agent_tui::modules::agent::application::usecases::GenerateResponseUseCase;

let use_case = container.generate_response_use_case();
let response = use_case.execute("Explain Rust ownership".to_string(), config).await?;
```

## Error Types

### ValidationError

```rust
pub enum ValidationError {
    EmptyName,
    InvalidProvider,
    InvalidModel,
    InvalidTemperature,
    InvalidMaxTokens,
}
```

### AgentError

```rust
pub enum AgentError {
    NotFound,
    Validation(ValidationError),
    Provider(AIError),
    Database(DbError),
}
```
