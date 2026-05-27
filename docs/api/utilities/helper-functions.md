---
title: Helper Functions
description: Utility functions แล helpers
---

# Helper Functions

## Overview

Helper functions สำหรับ common operations ทั่วทั้ง application

## String Utilities

### truncate

```rust
pub fn truncate(s: &str, max_len: usize) -> String
```

Truncate string ถ้ายาวเกิน max_len

**Example**:
```rust
let truncated = truncate("Hello World", 5);
assert_eq!(truncated, "Hello...");
```

### sanitize_filename

```rust
pub fn sanitize_filename(name: &str) -> String
```

Sanitize filename โดยลบ characters ที่ไม่ valid

**Example**:
```rust
let sanitized = sanitize_filename("file/name.txt");
assert_eq!(sanitized, "file_name.txt");
```

## Validation Utilities

### validate_path

```rust
pub fn validate_path(path: &str) -> Result<(), PathError>
```

Validate path สำหรับ security

**Example**:
```rust
validate_path("/etc/passwd")?; // Error: path traversal
validate_path("./src/main.rs")?; // OK
```

### validate_command

```rust
pub fn validate_command(cmd: &str) -> Result<(), CommandError>
```

Validate shell command สำหรับ safety

**Example**:
```rust
validate_command("rm -rf /")?; // Error: dangerous command
validate_command("ls -la")?; // OK
```

## Time Utilities

### format_timestamp

```rust
pub fn format_timestamp(dt: DateTime<Utc>) -> String
```

Format datetime สำหรับ display

**Example**:
```rust
let formatted = format_timestamp(Utc::now());
assert_eq!(formatted, "2024-01-01 12:00:00");
```

### parse_timestamp

```rust
pub fn parse_timestamp(s: &str) -> Result<DateTime<Utc>, ParseError>
```

Parse timestamp string

**Example**:
```rust
let dt = parse_timestamp("2024-01-01 12:00:00")?;
```

## File Utilities

### read_file_safe

```rust
pub async fn read_file_safe(path: &Path) -> Result<String, FileError>
```

Read file พร้อม path validation

**Example**:
```rust
let content = read_file_safe(Path::new("src/main.rs")).await?;
```

### write_file_safe

```rust
pub async fn write_file_safe(path: &Path, content: &str) -> Result<(), FileError>
```

Write file พร้อม path validation

**Example**:
```rust
write_file_safe(Path::new("output.txt"), "Hello").await?;
```

## Error Utilities

### to_app_error

```rust
pub fn to_app_error<E: std::error::Error>(err: E) -> AppError
```

Convert any error ไปเป็น AppError

**Example**:
```rust
let app_err = to_app_error(io_error);
```

### format_error

```rust
pub fn format_error(err: &dyn std::error::Error) -> String
```

Format error สำหรับ display

**Example**:
```rust
let formatted = format_error(&app_err);
```

## Async Utilities

### retry

```rust
pub async fn retry<F, Fut, T, E>(
    f: F,
    max_attempts: u32,
    delay: Duration,
) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
```

Retry async operation พร้อม exponential backoff

**Example**:
```rust
let result = retry(
    || async { api_call().await },
    3,
    Duration::from_secs(1),
).await?;
```

### timeout

```rust
pub async fn timeout<Fut, T>(
    future: Fut,
    duration: Duration,
) -> Result<T, TimeoutError>
where
    Fut: Future<Output = T>,
```

Timeout async operation

**Example**:
```rust
let result = timeout(api_call(), Duration::from_secs(5)).await?;
```

## Collection Utilities

### dedup

```rust
pub fn dedup<T: Eq + Hash + Clone>(vec: Vec<T>) -> Vec<T>
```

Remove duplicates จาก vector

**Example**:
```rust
let unique = dedup(vec![1, 2, 2, 3]);
assert_eq!(unique, vec![1, 2, 3]);
```

### chunk

```rust
pub fn chunk<T>(vec: Vec<T>, size: usize) -> Vec<Vec<T>>
```

Split vector ออกเป็น chunks

**Example**:
```rust
let chunks = chunk(vec![1, 2, 3, 4, 5], 2);
assert_eq!(chunks, vec![vec![1, 2], vec![3, 4], vec![5]]);
```

## Cryptographic Utilities

### hash_string

```rust
pub fn hash_string(s: &str) -> String
```

Hash string ด้วย SHA-256

**Example**:
```rust
let hash = hash_string("hello");
```

### generate_id

```rust
pub fn generate_id() -> String
```

Generate unique ID

**Example**:
```rust
let id = generate_id();
```

## Logging Utilities

### log_error

```rust
pub fn log_error(err: &dyn std::error::Error)
```

Log error พร้อม context

**Example**:
```rust
log_error(&app_err);
```

### log_info

```rust
pub fn log_info(message: &str)
```

Log info message

**Example**:
```rust
log_info("Application started");
```
