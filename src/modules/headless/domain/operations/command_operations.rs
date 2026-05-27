use crate::modules::headless::domain::models::command::{CommandType, OutputFormat};

/// Pure function to parse command from input string
pub fn parse_command(input: &str) -> Result<CommandType, String> {
    let input_lower = input.trim().to_lowercase();
    
    if input_lower.starts_with("/chat") || input.starts_with("chat") {
        return Ok(CommandType::Chat);
    }
    if input_lower.starts_with("/read") || input.starts_with("read") {
        return Ok(CommandType::FileRead);
    }
    if input_lower.starts_with("/write") || input.starts_with("write") {
        return Ok(CommandType::FileWrite);
    }
    if input_lower.starts_with("/exec") || input.starts_with("exec") || input.starts_with("!") {
        return Ok(CommandType::CommandExecute);
    }
    if input_lower.starts_with("/list") || input.starts_with("list") {
        return Ok(CommandType::SessionList);
    }
    if input_lower.starts_with("/create") || input.starts_with("create") {
        return Ok(CommandType::SessionCreate);
    }
    if input_lower.starts_with("/load") || input.starts_with("load") {
        return Ok(CommandType::SessionLoad);
    }
    if input_lower.starts_with("/help") || input.starts_with("help") || input == "?" {
        return Ok(CommandType::Help);
    }
    if input_lower.starts_with("/exit") || input_lower.starts_with("exit") || input_lower == "quit" {
        return Ok(CommandType::Exit);
    }
    
    // Default to chat if no command prefix
    Ok(CommandType::Chat)
}

/// Pure function to format output based on config
pub fn format_output(output: &str, format: &OutputFormat, include_metadata: bool) -> String {
    match format {
        OutputFormat::Text => {
            if include_metadata {
                format!("{}\n---\nFormat: Text", output)
            } else {
                output.to_string()
            }
        }
        OutputFormat::Json => {
            let json = serde_json::json!({
                "output": output,
                "format": "json",
                "timestamp": chrono::Utc::now().to_rfc3339()
            });
            serde_json::to_string_pretty(&json).unwrap_or_else(|_| output.to_string())
        }
        OutputFormat::Markdown => {
            if include_metadata {
                format!("{}\n\n---\n*Generated at {}*", output, chrono::Utc::now().to_rfc3339())
            } else {
                output.to_string()
            }
        }
    }
}

/// Pure function to truncate output if needed
pub fn truncate_output(output: &str, max_length: usize) -> String {
    if max_length == 0 || output.len() <= max_length {
        return output.to_string();
    }
    
    format!("{}... (truncated)", &output[..max_length])
}

/// Pure function to extract arguments from command input
pub fn extract_arguments(input: &str) -> Vec<String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return Vec::new();
    }
    
    // Skip the command part
    parts[1..].iter().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_command_chat() {
        assert_eq!(parse_command("/chat hello"), Ok(CommandType::Chat));
        assert_eq!(parse_command("chat hello"), Ok(CommandType::Chat));
    }

    #[test]
    fn test_parse_command_read() {
        assert_eq!(parse_command("/read file.txt"), Ok(CommandType::FileRead));
    }

    #[test]
    fn test_parse_command_default() {
        assert_eq!(parse_command("hello world"), Ok(CommandType::Chat));
    }

    #[test]
    fn test_format_output_text() {
        let output = format_output("test", &OutputFormat::Text, false);
        assert_eq!(output, "test");
    }

    #[test]
    fn test_format_output_json() {
        let output = format_output("test", &OutputFormat::Json, false);
        assert!(output.contains("\"output\""));
        assert!(output.contains("test"));
    }

    #[test]
    fn test_truncate_output() {
        let output = truncate_output("hello world", 5);
        assert_eq!(output, "hello... (truncated)");
    }

    #[test]
    fn test_extract_arguments() {
        let args = extract_arguments("/read file.txt --line 10");
        assert_eq!(args, vec!["file.txt", "--line", "10"]);
    }
}
