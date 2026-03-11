use crate::app::App;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn render_stats_content(f: &mut Frame, area: Rect, _app: &mut App) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(42, 42, 42)))
        .title(" Stats ");
    
    let content = Paragraph::new("\n\n  📊 Statistics are not yet implemented.\n\n  Press Tab to go back to Tasks.")
        .style(Style::default().fg(Color::Rgb(136, 136, 136)))
        .block(block);
        
    f.render_widget(content, area);
}
