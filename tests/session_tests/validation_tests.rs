use super::*;

#[test]
fn test_validate_session_name_valid() {
    assert!(validate_session_name("Valid Name").is_ok());
    assert!(validate_session_name("Test Session 123").is_ok());
    assert!(validate_session_name("a").is_ok());
}

#[test]
fn test_validate_session_name_empty() {
    assert!(matches!(
        validate_session_name(""),
        Err(SessionValidationError::EmptyName)
    ));
    assert!(matches!(
        validate_session_name("   "),
        Err(SessionValidationError::EmptyName)
    ));
}

#[test]
fn test_validate_session_name_too_long() {
    let long_name = "a".repeat(256);
    assert!(matches!(
        validate_session_name(&long_name),
        Err(SessionValidationError::NameTooLong)
    ));
}

#[test]
fn test_validate_session_name_invalid_chars() {
    assert!(matches!(
        validate_session_name("test/name"),
        Err(SessionValidationError::InvalidCharacters)
    ));
    assert!(matches!(
        validate_session_name("test\\name"),
        Err(SessionValidationError::InvalidCharacters)
    ));
}

#[test]
fn test_create_session_valid() {
    let session = create_session("New Session".to_string());
    assert!(session.is_ok());
    assert_eq!(session.unwrap().name, "New Session");
}

#[test]
fn test_create_session_invalid() {
    let result = create_session("".to_string());
    assert!(result.is_err());
}

#[test]
fn test_session_validation_error_display() {
    assert_eq!(
        format!("{}", SessionValidationError::EmptyName),
        "Session name cannot be empty"
    );
    assert_eq!(
        format!("{}", SessionValidationError::NameTooLong),
        "Session name cannot exceed 255 characters"
    );
    assert_eq!(
        format!("{}", SessionValidationError::InvalidCharacters),
        "Session name contains invalid characters"
    );
}
