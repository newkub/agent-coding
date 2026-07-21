use serde::{Deserialize, Serialize};

// mod tab_diagram;
// pub use tab_diagram::draw_all_tabs_diagram;

/// Additional tab ideas for future expansion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TabIdea {
    pub name: &'static str,
    pub description: &'static str,
    pub left_column: &'static str,
    pub center_column: &'static str,
    pub right_column: &'static str,
    pub priority: u8,             // 1-5, lower is higher priority
    pub complexity: &'static str, // Low, Medium, High
}

pub(crate) fn get_tab_ideas() -> Vec<TabIdea> {
    vec![
        TabIdea {
            name: "Search",
            description: "Global search across files, commands, snippets, and history",
            left_column: "Query Input",
            center_column: "Results",
            right_column: "Filters",
            priority: 1,
            complexity: "Medium",
        },
        TabIdea {
            name: "History",
            description: "Timeline of all actions, commands, and changes",
            left_column: "Timeline",
            center_column: "Details",
            right_column: "Actions",
            priority: 2,
            complexity: "Low",
        },
        TabIdea {
            name: "Notifications",
            description: "Alerts, warnings, and system notifications",
            left_column: "Categories",
            center_column: "Notifications",
            right_column: "Settings",
            priority: 2,
            complexity: "Low",
        },
        TabIdea {
            name: "Plugins",
            description: "Manage extensions and custom integrations",
            left_column: "Installed",
            center_column: "Store",
            right_column: "Config",
            priority: 3,
            complexity: "High",
        },
        TabIdea {
            name: "Analytics",
            description: "Metrics, insights, and productivity statistics",
            left_column: "Dashboard",
            center_column: "Charts",
            right_column: "Reports",
            priority: 4,
            complexity: "Medium",
        },
        TabIdea {
            name: "Terminal",
            description: "Full terminal emulator with multiple sessions",
            left_column: "Sessions",
            center_column: "Terminal",
            right_column: "Output",
            priority: 1,
            complexity: "High",
        },
        TabIdea {
            name: "Debug",
            description: "Debugging tools and process inspection",
            left_column: "Processes",
            center_column: "Inspector",
            right_column: "Console",
            priority: 3,
            complexity: "High",
        },
        TabIdea {
            name: "Resources",
            description: "System resources, CPU, memory, network",
            left_column: "Overview",
            center_column: "Charts",
            right_column: "Alerts",
            priority: 3,
            complexity: "Medium",
        },
        TabIdea {
            name: "Bookmarks",
            description: "Quick access to frequently used items",
            left_column: "Folders",
            center_column: "Bookmarks",
            right_column: "Tags",
            priority: 2,
            complexity: "Low",
        },
        TabIdea {
            name: "Tasks",
            description: "Task management and todo lists",
            left_column: "Projects",
            center_column: "Tasks",
            right_column: "Details",
            priority: 2,
            complexity: "Medium",
        },
    ]
}
