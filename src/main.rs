use std::{
    io,
    time::Duration,
    thread
};
use tui::{
    backend::CrosstermBackend,
    Terminal,
    widgets::{
        Block,
        List,
        Table,
        Paragraph,
        Tabs,
        Borders,
        Widget,
    },
    layout::{
        Layout,
        Constraint,
        Direction,
    },
};
use crossterm::{
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode,
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

// Constants
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), io::Error> {
    // Set up the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = 
        CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title(format!("FutureOS {}", VERSION))
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_millis(5000));

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    Ok(())
}
