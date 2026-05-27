//! Tab Content tests

use agent_tui::modules::ui::domain::models::TabContent;
use agent_tui::shared::kernel::types::Tab;

#[test]
fn test_tab_content_new() {
    let content = TabContent::new(Tab::Agent);
    assert!(content.left.is_empty());
    assert!(content.center.is_empty());
    assert!(content.right.is_empty());
}

#[test]
fn test_tab_content_with_content() {
    let content = TabContent::with_content(Tab::Agent, "Left", "Center", "Right");
    assert_eq!(content.left, "Left");
    assert_eq!(content.center, "Center");
    assert_eq!(content.right, "Right");
}
