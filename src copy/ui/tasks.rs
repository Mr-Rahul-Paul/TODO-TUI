use crate::app::App;
use crate::data::{get_class_color, get_priority_color, CLASSES};
use crate::types::{Panel, Priority, Tab, Task};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_tasks_content(f: &mut Frame, area: ratatui::layout::Rect, app: &mut App) {
    // Main layout
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(26), Constraint::Min(0)].as_ref())
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(7)].as_ref())
        .split(main_chunks[0]);

    // 2. Classes Panel
    let class_border_color = if let Panel::Classes = app.panel {
        Color::White
    } else {
        Color::Rgb(42, 42, 42)
    };
    let class_title_color = if let Panel::Classes = app.panel {
        Color::White
    } else {
        Color::Rgb(85, 85, 85)
    };
    let class_title = if let Panel::Classes = app.panel {
        "[1] Classes"
    } else {
        " 1  Classes"
    };

    let items: Vec<ListItem> = CLASSES
        .iter()
        .enumerate()
        .map(|(i, &cls)| {
            let count = if cls == "all" {
                app.tasks.len()
            } else {
                app.tasks.iter().filter(|t| t.cls == cls).count()
            };

            let is_selected = app.class_list_state.selected() == Some(i);
            let name_color = if is_selected {
                Color::White
            } else {
                Color::Rgb(136, 136, 136)
            };
            let count_color = if is_selected {
                Color::White
            } else {
                Color::Rgb(68, 68, 68)
            };

            let mut text = String::from("● ");
            while text.len() < 2 {
                text.push(' ');
            }
            let mut name = String::from(cls);
            while name.len() < 12 {
                name.push(' ');
            }
            let count_str = format!("{:>2}", count);

            let spans = vec![
                Span::styled("● ", Style::default().fg(get_class_color(cls))),
                Span::styled(name, Style::default().fg(name_color)),
                Span::styled(count_str, Style::default().fg(count_color)),
            ];

            if is_selected {
                ListItem::new(Line::from(spans)).style(Style::default().bg(Color::Rgb(26, 31, 46)))
            } else {
                ListItem::new(Line::from(spans))
            }
        })
        .collect();

    let classes_list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(class_border_color))
            .title(Span::styled(class_title, Style::default().fg(class_title_color))),
    );

    f.render_stateful_widget(classes_list, left_chunks[0], &mut app.class_list_state);

    // 3. Links Panel
    let links = vec![
        ListItem::new(Line::from(vec![Span::styled(
            " 2  Links\n",
            Style::default().fg(Color::Rgb(85, 85, 85)),
        )])),
        ListItem::new(Line::from(vec![
            Span::styled("⌥ ", Style::default().fg(Color::Rgb(85, 85, 85))),
            Span::styled(
                "github.com/Mr-Rahul-Paul",
                Style::default().fg(Color::Rgb(102, 102, 102)),
            ),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("◈ ", Style::default().fg(Color::Rgb(85, 85, 85))),
            Span::styled(
                "portfolio.dev",
                Style::default().fg(Color::Rgb(102, 102, 102)),
            ),
        ])),
        ListItem::new(Line::from(vec![
            Span::styled("◉ ", Style::default().fg(Color::Rgb(85, 85, 85))),
            Span::styled(
                "@Rahul_Paul",
                Style::default().fg(Color::Rgb(102, 102, 102)),
            ),
        ])),
    ];
    let links_list = List::new(links).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(42, 42, 42))),
    );
    f.render_widget(links_list, left_chunks[1]);

    // 4. Tasks Panel
    let task_border_color = if let Panel::Tasks = app.panel {
        Color::White
    } else {
        Color::Rgb(42, 42, 42)
    };
    let task_title_color = if let Panel::Tasks = app.panel {
        Color::White
    } else {
        Color::Rgb(85, 85, 85)
    };
    let task_title = if let Panel::Tasks = app.panel {
        "[3] Tasks -"
    } else {
        " 3  Tasks -"
    };

    let visible_tasks = app.visible_tasks();
    let done_count = visible_tasks.iter().filter(|t| t.done).count();
    let total_count = visible_tasks.len();

    let sel_class = CLASSES[app.class_list_state.selected().unwrap_or(0)];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(task_border_color))
        .title(Line::from(vec![
            Span::styled(
                format!("{} ", task_title),
                Style::default().fg(task_title_color),
            ),
            Span::styled(
                format!("{}  ", sel_class),
                Style::default().fg(Color::Rgb(167, 139, 250)),
            ),
            Span::styled(
                format!("{}/{} done", done_count, total_count),
                Style::default().fg(Color::Rgb(68, 68, 68)),
            ),
        ]));

    let mut task_items = Vec::new();
    let pending: Vec<&Task> = visible_tasks.iter().filter(|t| !t.done).collect();
    let done: Vec<&Task> = visible_tasks.iter().filter(|t| t.done).collect();

    if !pending.is_empty() {
        task_items.push(ListItem::new(Line::from(vec![Span::styled(
            format!("  ● Pending  {}", pending.len()),
            Style::default().fg(Color::Rgb(102, 102, 102)),
        )])));

        for t in pending {
            let idx = visible_tasks.iter().position(|x| x.id == t.id).unwrap();
            let is_selected = app.panel_tasks_active() && app.task_list_state.selected() == Some(idx);
            let bg = if is_selected {
                Color::Rgb(15, 27, 45)
            } else {
                Color::Reset
            };
            let text_color = if is_selected {
                Color::White
            } else {
                Color::Rgb(204, 204, 204)
            };

            let mut text = String::from(&t.text);
            while text.len() < 36 {
                text.push(' ');
            }

            let mut date = String::from(&t.date);
            while date.len() < 8 {
                date.insert(0, ' ');
            }

            let prio_str = match t.priority {
                Priority::High => "high",
                Priority::Mid => "mid ",
                Priority::Low => "low ",
            };

            let spans = vec![
                Span::styled("[ ] ", Style::default().fg(Color::Rgb(68, 68, 68))),
                Span::styled(text, Style::default().fg(text_color)),
                Span::styled(prio_str, Style::default().fg(get_priority_color(&t.priority))),
                Span::styled(date, Style::default().fg(Color::Rgb(85, 85, 85))),
            ];
            task_items.push(ListItem::new(Line::from(spans)).style(Style::default().bg(bg)));
        }
    }

    if !done.is_empty() {
        task_items.push(ListItem::new(Line::from(vec![Span::styled(
            format!("  ✓ Done     {}", done.len()),
            Style::default().fg(Color::Rgb(85, 85, 85)),
        )])));

        for t in done {
            let idx = visible_tasks.iter().position(|x| x.id == t.id).unwrap();
            let is_selected = app.panel_tasks_active() && app.task_list_state.selected() == Some(idx);
            let bg = if is_selected {
                Color::Rgb(15, 27, 45)
            } else {
                Color::Reset
            };

            let mut text = String::from(&t.text);
            while text.len() < 36 {
                text.push(' ');
            }

            let mut date = String::from(&t.date);
            while date.len() < 8 {
                date.insert(0, ' ');
            }

            let spans = vec![
                Span::styled("[✓] ", Style::default().fg(Color::Rgb(74, 222, 128))),
                Span::styled(text, Style::default().fg(Color::Rgb(85, 85, 85))),
                Span::styled("    ", Style::default().fg(Color::Reset)),
                Span::styled(date, Style::default().fg(Color::Rgb(68, 68, 68))),
            ];
            task_items.push(ListItem::new(Line::from(spans)).style(Style::default().bg(bg)));
        }
    }

    task_items.push(ListItem::new(Line::from(vec![Span::styled(
        "  + press 'a' to add...",
        Style::default().fg(Color::Rgb(51, 51, 51)),
    )])));

    let task_list = List::new(task_items).block(block);
    f.render_stateful_widget(task_list, main_chunks[1], &mut app.task_list_state);
}
