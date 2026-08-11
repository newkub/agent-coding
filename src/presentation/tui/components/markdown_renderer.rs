use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Render markdown content to ratatui Spans with syntax highlighting
pub(crate) fn render_markdown(text: &str) -> Vec<Line<'static>> {
    let parser = Parser::new(text);
    let mut lines = Vec::new();
    let mut current_line: Vec<Span<'static>> = Vec::new();
    let mut in_code_block = false;
    let mut in_list = false;
    let mut list_level = 0;

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { level, .. } => {
                    let style = match level {
                        pulldown_cmark::HeadingLevel::H1 => Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                        pulldown_cmark::HeadingLevel::H2 => Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                        pulldown_cmark::HeadingLevel::H3 => Style::default()
                            .fg(Color::Blue)
                            .add_modifier(Modifier::BOLD),
                        _ => Style::default().fg(Color::Blue),
                    };
                    let level_num = match level {
                        pulldown_cmark::HeadingLevel::H1 => 1,
                        pulldown_cmark::HeadingLevel::H2 => 2,
                        pulldown_cmark::HeadingLevel::H3 => 3,
                        pulldown_cmark::HeadingLevel::H4 => 4,
                        pulldown_cmark::HeadingLevel::H5 => 5,
                        pulldown_cmark::HeadingLevel::H6 => 6,
                    };
                    current_line.push(Span::styled("#".repeat(level_num), style));
                    current_line.push(Span::raw(" "));
                }
                Tag::BlockQuote(_) => {
                    current_line.push(Span::styled("│ ", Style::default().fg(Color::Yellow)));
                }
                Tag::CodeBlock(kind) => {
                    in_code_block = true;
                    if let pulldown_cmark::CodeBlockKind::Fenced(lang) = kind {
                        let lang = lang.to_string();
                        if !lang.is_empty() {
                            lines.push(Line::from(Span::styled(
                                format!("┌─ {} ─", lang),
                                Style::default().fg(Color::Green),
                            )));
                        }
                    }
                }
                Tag::List(_) => {
                    in_list = true;
                    list_level += 1;
                }
                Tag::Item => {
                    let indent = "  ".repeat(list_level - 1);
                    current_line.push(Span::styled(
                        format!("{}• ", indent),
                        Style::default().fg(Color::Yellow),
                    ));
                }
                Tag::Emphasis => {
                    current_line.push(Span::styled(
                        "",
                        Style::default().add_modifier(Modifier::ITALIC),
                    ));
                }
                Tag::Strong => {
                    current_line.push(Span::styled(
                        "",
                        Style::default().add_modifier(Modifier::BOLD),
                    ));
                }
                Tag::Strikethrough => {
                    current_line.push(Span::styled(
                        "",
                        Style::default().add_modifier(Modifier::CROSSED_OUT),
                    ));
                }
                _ => {}
            },
            Event::End(tag) => match tag {
                TagEnd::Heading(_) | TagEnd::Paragraph => {
                    if !current_line.is_empty() {
                        lines.push(Line::from(current_line.clone()));
                        current_line.clear();
                    }
                    if matches!(tag, TagEnd::Paragraph) {
                        lines.push(Line::from(""));
                    }
                }
                TagEnd::CodeBlock => {
                    in_code_block = false;
                    lines.push(Line::from(Span::styled(
                        "└────────",
                        Style::default().fg(Color::Green),
                    )));
                    lines.push(Line::from(""));
                }
                TagEnd::List(_) => {
                    in_list = false;
                    list_level = list_level.saturating_sub(1);
                }
                TagEnd::Item if !current_line.is_empty() => {
                    lines.push(Line::from(current_line.clone()));
                    current_line.clear();
                }
                _ => {}
            },
            Event::Text(text) => {
                let style = if in_code_block {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };
                current_line.push(Span::styled(text.to_string(), style));
            }
            Event::Code(text) => {
                current_line.push(Span::styled(
                    format!("`{}`", text),
                    Style::default().fg(Color::Green),
                ));
            }
            Event::SoftBreak if !current_line.is_empty() => {
                lines.push(Line::from(current_line.clone()));
                current_line.clear();
            }
            Event::HardBreak => {
                if !current_line.is_empty() {
                    lines.push(Line::from(current_line.clone()));
                    current_line.clear();
                }
                lines.push(Line::from(""));
            }
            Event::Rule => {
                lines.push(Line::from(Span::styled(
                    "─".repeat(50),
                    Style::default().fg(Color::DarkGray),
                )));
            }
            _ => {}
        }
    }

    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
    }

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_markdown_heading() {
        let text = "# Heading 1";
        let result = render_markdown(text);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_render_markdown_code_block() {
        let text = "```rust\nfn main() {}\n```";
        let result = render_markdown(text);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_render_markdown_list() {
        let text = "- Item 1\n- Item 2";
        let result = render_markdown(text);
        assert!(!result.is_empty());
    }
}
