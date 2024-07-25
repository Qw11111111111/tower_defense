use {
    std::io::{self, stdout, Stdout},
    crossterm::{
        event::{EnableMouseCapture, DisableMouseCapture},
        execute,
        terminal::*
    },
    ratatui::prelude::*
};

pub type Tui = Terminal<CrosstermBackend<Stdout>>;

pub fn init () -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    execute!(stdout(), EnableMouseCapture)?;
    enable_raw_mode()?;
    Tui::new(CrosstermBackend::new(stdout())) 
}

pub fn restore() -> io::Result<()> {
    execute!(stdout(), DisableMouseCapture)?;
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}