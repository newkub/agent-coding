use crate::shared::kernel::result::AppResult;

/// Tutorial state for onboarding
#[derive(Debug, Clone, PartialEq)]
pub enum TutorialStep {
    Welcome,
    FirstMessage,
    Navigation,
    FileOperations,
    GitOperations,
    Settings,
    Complete,
}

impl TutorialStep {
    pub fn next(&self) -> Option<Self> {
        match self {
            TutorialStep::Welcome => Some(TutorialStep::FirstMessage),
            TutorialStep::FirstMessage => Some(TutorialStep::Navigation),
            TutorialStep::Navigation => Some(TutorialStep::FileOperations),
            TutorialStep::FileOperations => Some(TutorialStep::GitOperations),
            TutorialStep::GitOperations => Some(TutorialStep::Settings),
            TutorialStep::Settings => Some(TutorialStep::Complete),
            TutorialStep::Complete => None,
        }
    }

    pub fn previous(&self) -> Option<Self> {
        match self {
            TutorialStep::Welcome => None,
            TutorialStep::FirstMessage => Some(TutorialStep::Welcome),
            TutorialStep::Navigation => Some(TutorialStep::FirstMessage),
            TutorialStep::FileOperations => Some(TutorialStep::Navigation),
            TutorialStep::GitOperations => Some(TutorialStep::FileOperations),
            TutorialStep::Settings => Some(TutorialStep::GitOperations),
            TutorialStep::Complete => Some(TutorialStep::Settings),
        }
    }

    pub fn title(&self) -> &'static str {
        match self {
            TutorialStep::Welcome => "Welcome to agent-tui",
            TutorialStep::FirstMessage => "Send Your First Message",
            TutorialStep::Navigation => "Navigate the Interface",
            TutorialStep::FileOperations => "File Operations",
            TutorialStep::GitOperations => "Git Operations",
            TutorialStep::Settings => "Configure Settings",
            TutorialStep::Complete => "Tutorial Complete",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            TutorialStep::Welcome => "Learn the basics of agent-tui in 5 minutes",
            TutorialStep::FirstMessage => "Type a message and press Enter to chat with AI",
            TutorialStep::Navigation => "Use Tab to switch between different panels",
            TutorialStep::FileOperations => "Read, write, and manage files",
            TutorialStep::GitOperations => "View status, commit changes, and manage branches",
            TutorialStep::Settings => "Configure AI providers and app preferences",
            TutorialStep::Complete => "You're ready to use agent-tui!",
        }
    }

    pub fn hint(&self) -> &'static str {
        match self {
            TutorialStep::Welcome => "Press Enter to continue",
            TutorialStep::FirstMessage => "Type 'Hello' and press Enter",
            TutorialStep::Navigation => "Press Tab to switch tabs",
            TutorialStep::FileOperations => "Type 'Read README.md' and press Enter",
            TutorialStep::GitOperations => "Type 'Show git status' and press Enter",
            TutorialStep::Settings => "Press Tab to go to Settings tab",
            TutorialStep::Complete => "Press Escape to exit tutorial",
        }
    }
}

/// Tutorial manager
pub struct TutorialManager {
    current_step: TutorialStep,
    completed: bool,
}

impl TutorialManager {
    pub fn new() -> Self {
        Self {
            current_step: TutorialStep::Welcome,
            completed: false,
        }
    }

    pub fn current_step(&self) -> &TutorialStep {
        &self.current_step
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn next_step(&mut self) -> AppResult<()> {
        if let Some(next) = self.current_step.next() {
            self.current_step = next;
        } else {
            self.completed = true;
        }
        Ok(())
    }

    pub fn previous_step(&mut self) -> AppResult<()> {
        if let Some(prev) = self.current_step.previous() {
            self.current_step = prev;
        }
        Ok(())
    }

    pub fn skip(&mut self) {
        self.completed = true;
    }

    pub fn reset(&mut self) {
        self.current_step = TutorialStep::Welcome;
        self.completed = false;
    }
}

impl Default for TutorialManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tutorial_flow() {
        let mut tutorial = TutorialManager::new();
        
        assert_eq!(tutorial.current_step(), &TutorialStep::Welcome);
        
        tutorial.next_step().unwrap();
        assert_eq!(tutorial.current_step(), &TutorialStep::FirstMessage);
        
        tutorial.next_step().unwrap();
        assert_eq!(tutorial.current_step(), &TutorialStep::Navigation);
        
        tutorial.previous_step().unwrap();
        assert_eq!(tutorial.current_step(), &TutorialStep::FirstMessage);
    }

    #[test]
    fn test_tutorial_completion() {
        let mut tutorial = TutorialManager::new();
        
        // Go through all steps
        for _ in 0..7 {
            tutorial.next_step().unwrap();
        }
        
        assert!(tutorial.is_completed());
    }

    #[test]
    fn test_tutorial_skip() {
        let mut tutorial = TutorialManager::new();
        tutorial.skip();
        
        assert!(tutorial.is_completed());
    }

    #[test]
    fn test_tutorial_reset() {
        let mut tutorial = TutorialManager::new();
        tutorial.skip();
        tutorial.reset();
        
        assert!(!tutorial.is_completed());
        assert_eq!(tutorial.current_step(), &TutorialStep::Welcome);
    }
}
