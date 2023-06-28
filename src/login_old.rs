use std::error::Error;

use crate::VERSION;
use tui::{
    backend::Backend,
    Frame,
    layout::{Layout, Direction, Constraint},
    widgets::{Block, Borders}
};

// App holds the state of the login screen
struct App {
    // Current value of the input box
    input: String,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
        }
    }
}

pub fn render_login_screen<B: Backend>(f: &mut Frame<B>) -> 
    Result<(), Box<dyn Error>> {
    // Create App and run it
    let app = App::default();
    // let res =   run_app(&mut terminal);
    
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    // let chunks = Layout::default()
    //     .direction(Direction::Vertical)
    //     .margin(1)
    //     .constraints(
    //         [
    //             Constraint::Percentage(10),
    //             Constraint::Percentage(80),
    //             Constraint::Percentage(10),
    //         ].as_ref()
    //     )
    //     .split(f.size());

    let size = f.size();

    let block = Block::default()
        .title(format!("FutureOS v{}", VERSION))
        .borders(Borders::ALL);

    f.render_widget(block, size);
}