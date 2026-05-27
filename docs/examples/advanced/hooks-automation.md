---
title: Hooks Automation Example
description: Hooks automation examples
---

# Hooks Automation Example

## Overview

ตัวอย่างการใช้ hooks สำหรับ automation ใน agent-tui

## Pre-Commit Hook

### Define Hook

```rust
use agent_tui::modules::hooks::domain::models::Hook;

let hook = Hook {
    name: "pre-commit-review".to_string(),
    trigger: HookTrigger::PreCommit,
    action: HookAction::RunSkill {
        skill_name: "code-review".to_string(),
        parameters: vec![],
    },
    condition: HookCondition::Always,
};
```

### Register Hook

```rust
use agent_tui::presentation::di::DIContainer;

let container = DIContainer::new();
let use_case = container.register_hook_use_case();

use_case.execute(hook).await?;
```

## Post-Message Hook

### Define Hook

```rust
let hook = Hook {
    name: "post-message-save".to_string(),
    trigger: HookTrigger::PostMessage,
    action: HookAction::SaveSession,
    condition: HookCondition::MessageLength { min: 100 },
};
```

## Custom Hook with Condition

```rust
let hook = Hook {
    name: "error-detection".to_string(),
    trigger: HookTrigger::PostMessage,
    action: HookAction::RunSkill {
        skill_name: "analyze-error".to_string(),
        parameters: vec![],
    },
    condition: HookCondition::ContainsError,
};
```

## Hook Chain

```rust
let hooks = vec![
    Hook {
        name: "validate".to_string(),
        trigger: HookTrigger::PreCommit,
        action: HookAction::Validate,
        condition: HookCondition::Always,
    },
    Hook {
        name: "review".to_string(),
        trigger: HookTrigger::PreCommit,
        action: HookAction::RunSkill {
            skill_name: "code-review".to_string(),
            parameters: vec![],
        },
        condition: HookCondition::OnSuccess { previous_hook: "validate".to_string() },
    },
];
```

## Expected Output

```
Hook: pre-commit-review
Triggered: Pre-commit
Action: Running skill code-review
Result: Success
```
