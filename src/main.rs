pub mod app;
pub mod data;
pub mod types;
pub mod ui;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};
use types::Tab;

// ─── Main Execution ───────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Set panic hook to restore terminal if we crash
    let panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture);
        panic_hook(panic_info);
    }));

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    io::Error: From<<B as Backend>::Error>,
{
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        match event::read()? {
            Event::Key(key) => {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Tab => {
                            app.panel = match app.panel {
                                types::Panel::Classes => types::Panel::Tasks,
                                types::Panel::Tasks => types::Panel::Classes,
                            }
                        }
                        KeyCode::Down | KeyCode::Char('j') => app.next_item(),
                        KeyCode::Up | KeyCode::Char('k') => app.previous_item(),
                        KeyCode::Char(' ') => app.toggle_task(),
                        KeyCode::Char('d') => app.delete_task(),
                        KeyCode::Char('a') => app.add_task(),
                        _ => {}
                    }
                }
            }
            Event::Mouse(mouse_event) => {
                use crossterm::event::{MouseButton, MouseEventKind};

                let should_process = match mouse_event.kind {
                    MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Moved => true,
                    _ => false,
                };

                if should_process {
                    let (x, y) = (mouse_event.column, mouse_event.row);

                    // Top bar is at y == 0
                    if y == 0 {
                        if x >= 12 && x <= 16 {
                            app.active_tab = Tab::Tasks;
                        } else if x >= 19 && x <= 23 {
                            app.active_tab = Tab::Stats;
                        } else if x >= 26 && x <= 31 {
                            app.active_tab = Tab::Config;
                        }
                    } else {
                        // Region below top bar
                        // left_chunks is length 26
                        if x < 26 {
                            app.panel = types::Panel::Classes;
                        } else {
                            app.panel = types::Panel::Tasks;
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
