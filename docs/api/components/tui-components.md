---
title: TUI Components API
description: TUI component APIs แล props
---

# TUI Components API

## Overview

TUI components ใช้ ratatui framework สำหรับ terminal UI rendering

## Core Components

### App

```rust
pub struct App {
    pub sessions: Vec<Session>,
    pub current_session: Option<Session>,
    pub input: String,
    pub messages: Vec<Message>,
    pub mode: AppMode,
}

pub enum AppMode {
    Normal,
    Insert,
    Command,
}
```

### Renderer

```rust
pub struct Renderer {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Renderer {
    pub fn new() -> Result<Self>
    pub fn draw(&mut self, app: &App) -> Result<()>
    pub fn cleanup(&mut self) -> Result<()>
}
```

## UI Components

### InputComponent

```rust
pub struct InputComponent {
    pub input: String,
    pub cursor_position: usize,
    pub placeholder: String,
}

impl InputComponent {
    pub fn new(placeholder: String) -> Self
    pub fn handle_char(&mut self, c: char)
    pub fn handle_key(&mut self, key: Key)
    pub fn render(&self, area: Rect) -> Paragraph
}
```

### MessageListComponent

```rust
pub struct MessageListComponent {
    pub messages: Vec<Message>,
    pub scroll_offset: usize,
}

impl MessageListComponent {
    pub fn new(messages: Vec<Message>) -> Self
    pub fn scroll_down(&mut self)
    pub fn scroll_up(&mut self)
    pub fn render(&self, area: Rect) -> List
}
```

### SessionListComponent

```rust
pub struct SessionListComponent {
    pub sessions: Vec<Session>,
    pub selected_index: usize,
    pub scroll_offset: usize,
}

impl SessionListComponent {
    pub fn new(sessions: Vec<Session>) -> Self
    pub fn select_next(&mut self)
    pub fn select_previous(&mut self)
    pub fn render(&self, area: Rect) -> Table
}
```

### StatusComponent

```rust
pub struct StatusComponent {
    pub status: String,
    pub mode: AppMode,
    pub session_name: Option<String>,
}

impl StatusComponent {
    pub fn new(status: String) -> Self
    pub fn set_status(&mut self, status: String)
    pub fn render(&self, area: Rect) -> Paragraph
}
```

## Event Handling

### EventHandler

```rust
pub struct EventHandler {
    pub tx: UnboundedSender<Event>,
}

impl EventHandler {
    pub fn new(tx: UnboundedSender<Event>) -> Self
    pub fn handle_key(&self, key: Key)
    pub fn handle_char(&self, c: char)
    pub fn handle_mouse(&self, event: MouseEvent)
}
```

### Event

```rust
pub enum Event {
    Input(KeyEvent),
    Tick,
    Message(Message),
    SessionUpdate(Session),
}
```

## Usage Examples

### Initialize TUI

```rust
use agent_tui::presentation::tui::app::App;
use agent_tui::presentation::tui::renderer::Renderer;

let mut app = App::new();
let mut renderer = Renderer::new()?;

loop {
    renderer.draw(&app)?;
    // Handle events
    app.update();
}
```

### Handle Input

```rust
use agent_tui::presentation::tui::components::InputComponent;

let mut input = InputComponent::new("Enter message...".to_string());

input.handle_char('H');
input.handle_char('e');
input.handle_char('l');
input.handle_char('l');
input.handle_char('o');
```

### Render Messages

```rust
use agent_tui::presentation::tui::components::MessageListComponent;

let messages = vec![/* ... */];
let component = MessageListComponent::new(messages);

let paragraph = component.render(area);
frame.render_widget(paragraph, area);
```

## Styles

### Theme

```rust
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub background: Color,
    pub foreground: Color,
}

impl Theme {
    pub fn default() -> Self
    pub fn dark() -> Self
    pub fn light() -> Self
}
```

### StyleConfig

```rust
pub struct StyleConfig {
    pub theme: Theme,
    pub border_style: Style,
    pub highlight_style: Style,
}
```

## Keyboard Shortcuts

### Key Bindings

```rust
pub struct KeyBindings {
    pub quit: Vec<Key>,
    pub save: Vec<Key>,
    pub new_session: Vec<Key>,
    pub list_sessions: Vec<Key>,
}

impl KeyBindings {
    pub fn default() -> Self
}
```

## Error Handling

### TUIError

```rust
pub enum TUIError {
    TerminalError(io::Error),
    RenderError(String),
    InputError(String),
}
```
