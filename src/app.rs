use crate::data::{seed_tasks, CLASSES};
use crate::types::{Panel, Task, Tab};
use ratatui::widgets::ListState;

pub struct App {
    pub tasks: Vec<Task>,
    pub panel: Panel,
    pub class_list_state: ListState,
    pub task_list_state: ListState,
    pub active_tab: Tab,
}

impl App {
    pub fn new() -> App {
        let mut class_list_state = ListState::default();
        class_list_state.select(Some(0));

        let mut task_list_state = ListState::default();
        task_list_state.select(Some(0));

        App {
            tasks: seed_tasks(),
            panel: Panel::Tasks,
            class_list_state,
            task_list_state,
            active_tab: Tab::Tasks,
        }
    }

    pub fn visible_tasks(&self) -> Vec<Task> {
        let sel_idx = self.class_list_state.selected().unwrap_or(0);
        let cls = CLASSES[sel_idx];
        if cls == "all" {
            self.tasks.clone()
        } else {
            self.tasks.iter().filter(|t| t.cls == cls).cloned().collect()
        }
    }

    // Helper for moving list selection down
    fn next_index(state: &mut ListState, len: usize) {
        if len == 0 { return; }
        let i = state.selected().map_or(0, |i| (i + 1).min(len - 1));
        state.select(Some(i));
    }

    // Helper for moving list selection up
    fn prev_index(state: &mut ListState) {
        let i = state.selected().map_or(0, |i| i.saturating_sub(1));
        state.select(Some(i));
    }

    pub fn next_item(&mut self) {
        match self.panel {
            Panel::Classes => {
                Self::next_index(&mut self.class_list_state, CLASSES.len());
                self.task_list_state.select(Some(0));
            }
            Panel::Tasks => {
                let len = self.visible_tasks().len();
                Self::next_index(&mut self.task_list_state, len);
            }
        }
    }

    pub fn previous_item(&mut self) {
        match self.panel {
            Panel::Classes => {
                Self::prev_index(&mut self.class_list_state);
                self.task_list_state.select(Some(0));
            }
            Panel::Tasks => Self::prev_index(&mut self.task_list_state),
        }
    }

    pub fn toggle_task(&mut self) {
        if self.panel == Panel::Classes { return; }
        
        if let Some(i) = self.task_list_state.selected() {
            if let Some(t) = self.visible_tasks().get(i) {
                let id = t.id;
                if let Some(task) = self.tasks.iter_mut().find(|x| x.id == id) {
                    task.done = !task.done;
                }
            }
        }
    }

    pub fn delete_task(&mut self) {
        if self.panel == Panel::Classes { return; }
        
        if let Some(i) = self.task_list_state.selected() {
            if let Some(t) = self.visible_tasks().get(i) {
                let id = t.id;
                self.tasks.retain(|x| x.id != id);
                if i > 0 { self.task_list_state.select(Some(i - 1)); }
            }
        }
    }

    pub fn add_task(&mut self) {
        if self.panel == Panel::Classes { return; }
        
        let sel_idx = self.class_list_state.selected().unwrap_or(0);
        let cls = CLASSES[sel_idx];
        let new_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        
        self.tasks.push(Task {
            id: new_id,
            text: "New task".into(),
            cls: cls.into(),
            priority: crate::types::Priority::Mid,
            done: false,
            date: "today".into(),
        });
        
        let visible_count = self.visible_tasks().len();
        if visible_count > 0 {
            self.task_list_state.select(Some(visible_count - 1));
        }
    }
    
    pub fn panel_tasks_active(&self) -> bool {
        matches!(self.panel, Panel::Tasks)
    }
}
