pub mod config;
pub mod stats;
pub mod tasks;

use crate::app::App;
use crate::types::Tab;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(f.area());

    // 1. Title Bar
    let (task_color, stats_color, config_color) = match app.active_tab {
        Tab::Tasks => (Color::White, Color::DarkGray, Color::DarkGray),
        Tab::Stats => (Color::DarkGray, Color::White, Color::DarkGray),
        Tab::Config => (Color::DarkGray, Color::DarkGray, Color::White),
    };

    let title = Line::from(vec![
        Span::styled(
            "◆ todo-tui ",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("Tasks  ", Style::default().fg(task_color)),
        Span::styled("Stats  ", Style::default().fg(stats_color)),
        Span::styled("Config", Style::default().fg(config_color)),
    ]);
    f.render_widget(
        Paragraph::new(title).style(Style::default().bg(Color::Rgb(17, 17, 17))),
        chunks[0],
    );

    // 2. Main Content
    match app.active_tab {
        Tab::Tasks => tasks::render_tasks_content(f, chunks[1], app),
        Tab::Stats => stats::render_stats_content(f, chunks[1], app),
        Tab::Config => config::render_config_content(f, chunks[1], app),
    }

    // 3. Status Bar
    let status_line = Line::from(vec![
        Span::styled(
            "[j/k]",
            Style::default().fg(Color::White).bg(Color::Rgb(26, 26, 26)),
        ),
        Span::styled(" navigate  ", Style::default().fg(Color::Rgb(85, 85, 85))),
        Span::styled(
            "[Space]",
            Style::default().fg(Color::White).bg(Color::Rgb(26, 26, 26)),
        ),
        Span::styled(" toggle  ", Style::default().fg(Color::Rgb(85, 85, 85))),
        Span::styled(
            "[d]",
            Style::default().fg(Color::White).bg(Color::Rgb(26, 26, 26)),
        ),
        Span::styled(" delete  ", Style::default().fg(Color::Rgb(85, 85, 85))),
        Span::styled(
            "[Tab]",
            Style::default().fg(Color::White).bg(Color::Rgb(26, 26, 26)),
        ),
        Span::styled(" switch  ", Style::default().fg(Color::Rgb(85, 85, 85))),
        Span::styled(
            "[q]",
            Style::default().fg(Color::White).bg(Color::Rgb(26, 26, 26)),
        ),
        Span::styled(" quit  ", Style::default().fg(Color::Rgb(85, 85, 85))),
    ]);
    f.render_widget(
        Paragraph::new(status_line).style(Style::default().bg(Color::Rgb(13, 13, 13))),
        chunks[2],
    );
}
