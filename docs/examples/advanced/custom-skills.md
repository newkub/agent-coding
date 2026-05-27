---
title: Custom Skills Example
description: Custom skills examples
---

# Custom Skills Example

## Overview

ตัวอย่างการสร้างและใช้ custom skills ใน agent-tui

## Create Skill

### Define Skill

```rust
use agent_tui::modules::skills::domain::models::Skill;

let skill = Skill {
    name: "code-review".to_string(),
    description: "Review code for best practices".to_string(),
    template: "Review the following code for best practices:\n\n{code}".to_string(),
    parameters: vec![
        SkillParameter {
            name: "code".to_string(),
            description: "Code to review".to_string(),
            required: true,
        },
    ],
};
```

### Register Skill

```rust
use agent_tui::presentation::di::DIContainer;

let container = DIContainer::new();
let use_case = container.register_skill_use_case();

use_case.execute(skill).await?;
```

## Use Skill

### TUI

ใน TUI chat, พิมพ์:

```
Use skill code-review with code:
fn main() {
    println!("Hello");
}
```

### Programmatic

```rust
use agent_tui::presentation::di::DIContainer;

let container = DIContainer::new();
let use_case = container.use_skill_use_case();

let parameters = vec![
    ("code".to_string(), "fn main() { println!(\"Hello\"); }".to_string()),
];

let result = use_case.execute("code-review".to_string(), parameters).await?;
```

## Advanced Skill with Multiple Parameters

```rust
let skill = Skill {
    name: "refactor".to_string(),
    description: "Refactor code with specific style".to_string(),
    template: "Refactor the following code to {style} style:\n\n{code}".to_string(),
    parameters: vec![
        SkillParameter {
            name: "code".to_string(),
            description: "Code to refactor".to_string(),
            required: true,
        },
        SkillParameter {
            name: "style".to_string(),
            description: "Target style (functional, oop, etc)".to_string(),
            required: true,
        },
    ],
};
```

## Skill with Context

```rust
let skill = Skill {
    name: "debug-with-context".to_string(),
    description: "Debug code with file context".to_string(),
    template: "Debug the following issue in {file}:\n\n{error}\n\nContext:\n{context}".to_string(),
    parameters: vec![
        SkillParameter {
            name: "file".to_string(),
            description: "File path".to_string(),
            required: true,
        },
        SkillParameter {
            name: "error".to_string(),
            description: "Error message".to_string(),
            required: true,
        },
        SkillParameter {
            name: "context".to_string(),
            description: "File context".to_string(),
            required: true,
        },
    ],
};
```

## Expected Output

```
Skill: code-review
Reviewing code for best practices...

✓ Code follows Rust naming conventions
✓ Error handling is present
⚠ Consider adding documentation
✓ No security issues found
```
