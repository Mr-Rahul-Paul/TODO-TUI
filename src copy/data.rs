pub const CLASSES: [&str; 5] = ["all", "classwork", "intern", "personal", "research"];

pub fn get_class_color(cls: &str) -> ratatui::style::Color {
    use ratatui::style::Color;
    match cls {
        "all" => Color::Rgb(74, 158, 255),
        "classwork" => Color::Rgb(167, 139, 250),
        "intern" => Color::Rgb(250, 204, 21),
        "personal" => Color::Rgb(74, 222, 128),
        "research" => Color::Rgb(248, 113, 113),
        _ => Color::White,
    }
}

pub fn get_priority_color(p: &crate::types::Priority) -> ratatui::style::Color {
    use ratatui::style::Color;
    use crate::types::Priority;
    match p {
        Priority::High => Color::Rgb(248, 113, 113),
        Priority::Mid => Color::Rgb(250, 204, 21),
        Priority::Low => Color::Rgb(74, 222, 128),
    }
}

pub fn seed_tasks() -> Vec<crate::types::Task> {
    use crate::types::{Priority, Task};
    vec![
        Task { id: 1, text: "Set up CI pipeline".into(), cls: "intern".into(), priority: Priority::High, done: false, date: "today".into() },
        Task { id: 2, text: "Finish DSA assignment - trees".into(), cls: "classwork".into(), priority: Priority::Mid, done: false, date: "Mar 09".into() },
        Task { id: 3, text: "Read chapter 4 - OS concepts".into(), cls: "classwork".into(), priority: Priority::Low, done: false, date: "Mar 10".into() },
        Task { id: 4, text: "Draft intern project proposal".into(), cls: "intern".into(), priority: Priority::High, done: false, date: "Mar 08".into() },
        Task { id: 5, text: "Update portfolio README".into(), cls: "personal".into(), priority: Priority::Low, done: false, date: "Mar 12".into() },
        Task { id: 6, text: "Review PR from teammate".into(), cls: "intern".into(), priority: Priority::Mid, done: false, date: "today".into() },
        Task { id: 7, text: "Write unit tests for auth".into(), cls: "research".into(), priority: Priority::Mid, done: false, date: "Mar 11".into() },
        Task { id: 8, text: "Set up opentui project".into(), cls: "personal".into(), priority: Priority::Low, done: true, date: "Mar 06".into() },
        Task { id: 9, text: "Submit internship application".into(), cls: "intern".into(), priority: Priority::High, done: true, date: "Mar 04".into() },
    ]
}
