mod app;
mod session;
mod ui;

use std::io;

use app::App;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

fn run() -> io::Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();

    loop {
        terminal.draw(|frame| ui::draw(frame, &app))?;

        if app.should_quit {
            break;
        }

        if let Event::Key(key) = event::read()? {
            app.handle_key(key);
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;

    let result = run();

    // Terminal auch bei Fehlern aus run() wiederherstellen.
    let raw_result = disable_raw_mode();
    let screen_result = execute!(io::stdout(), LeaveAlternateScreen);

    result.and(raw_result).and(screen_result)
}
