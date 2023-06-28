// Rust imports
use std::{
    io,
    time::Duration,
    thread
};
// External crates
use tui::{
    backend::{CrosstermBackend, Backend},
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
    }, Frame,
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

// Local modules
mod datetime;
mod login;
mod api;

// Constants
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), io::Error> {
    println!("Launching FutureOS v{}…", VERSION);

    // Set up the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = 
        CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;

    // terminal.draw(|f| {
    //     let size = f.size();
    //     let block = Block::default()
    //         .title(format!("FutureOS {}", VERSION))
    //         .borders(Borders::ALL);
    //     f.render_widget(block, size);
    // })?;

    // Render all UIs
    terminal.draw(|f| {
        // render_ui(f);
        login::main();
    })?;

    // thread::sleep(Duration::from_millis(8000));

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;

    println!("Finished!");

    Ok(())
}

fn render_ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ].as_ref()
        )
        .split(f.size());

    let block = Block::default()
        .title("Block")
        .borders(Borders::ALL);

    f.render_widget(block, chunks[0]);

    let block = Block::default()
        .title("Block 2")
        .borders(Borders::ALL);

    f.render_widget(block, chunks[1]);
}