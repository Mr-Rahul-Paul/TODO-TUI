#[derive(Clone)]
pub enum Priority {
    High,
    Mid,
    Low,
}

#[derive(Clone)]
pub struct Task {
    pub id: usize,
    pub text: String,
    pub cls: String,
    pub priority: Priority,
    pub done: bool,
    pub date: String,
}

pub enum Tab {
    Tasks,
    Stats,
    Config,
}

#[derive(Clone, PartialEq, Eq)]
pub enum Panel {
    Classes,
    Tasks,
}
