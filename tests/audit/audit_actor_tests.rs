//! Audit Actor tests

use agent_tui::modules::audit::domain::models::{Actor, ActorType};

#[test]
fn test_actor_type_variants() {
    assert!(matches!(ActorType::User, ActorType::User));
    assert!(matches!(ActorType::Ai, ActorType::Ai));
    assert!(matches!(ActorType::System, ActorType::System));
}
