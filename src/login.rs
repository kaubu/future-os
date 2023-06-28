use std::{error::Error, io, thread, time::Duration};
use crossterm::{
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode, poll, KeyEventKind
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal, terminal,
};
use unicode_width::UnicodeWidthStr;

const CENSOR_PASSWORD: bool = true;

// State of which input box is selected
enum SelectedInput {
    Username,
    Password,
}

// App holds the state of the application
struct App {
    // Current value of the username input box
    username_input: String,
    // Current value of the password input box
    password_input: String,
    // Censored password input to display
    censored_input: String,
    // Which input box is currently in use
    selected_input: SelectedInput,
}

impl Default for App {
    fn default() -> App {
        App {
            username_input: String::new(),
            password_input: String::new(),
            censored_input: String::new(),
            selected_input: SelectedInput::Username,
        }
    }
}

impl App {
    fn invert_selected_input(&mut self) {
        self.selected_input = match self.selected_input {
            SelectedInput::Username => {
                SelectedInput::Password
            },
            SelectedInput::Password => {
                SelectedInput::Username
            },
        }
    }

    fn insert_password_char(&mut self, c: char) {
        self.password_input.push(c);
        self.censored_input.push('*');
    }

    fn delete_password_char(&mut self) {
        self.password_input.pop();
        self.censored_input.pop();
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // Setup the login terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = 
        CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("ERROR::LOGIN: {:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> 
    io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        // Attempt to log in Return username and password to
                        // the main file For now, just exit return Ok(());
                    },
                    KeyCode::Backspace => {
                        match app.selected_input {
                            SelectedInput::Username => {
                                app.username_input.pop();
                                // All match arms have to return the same type
                                ()
                            }
                            SelectedInput::Password => {
                                app.delete_password_char();
                                // All match arms have to return the same type
                                ()
                            }
                        };
                    },
                    KeyCode::Tab => {
                        app.invert_selected_input();
                    },
                    KeyCode::Char('q') => {
                        return Ok(());
                    },
                    KeyCode::Char(c) => {
                        match app.selected_input {
                            SelectedInput::Username => {
                                app.username_input.push(c);
                                continue;
                            },
                            SelectedInput::Password => {
                                app.insert_password_char(c);
                                continue;
                            },
                        };
                    },
                    _ => {},
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                // The Min here is required in order for the password box not
                // to auto-extend to the bottom
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let username_input =
        Paragraph::new(app.username_input.as_str())
        .style(match app.selected_input {
            SelectedInput::Username => Style::default().fg(Color::Yellow),
            SelectedInput::Password => Style::default(),
        })
        .block(
            Block::default()
            .borders(Borders::ALL)
            .title("Username")
        )
        .wrap(Wrap { trim: true });
    f.render_widget(username_input, chunks[0]);

        let password_field;
        if CENSOR_PASSWORD {
            password_field = app.censored_input.as_str();
        } else {
            password_field = app.password_input.as_str();
        }
        
        let password_input =
            Paragraph::new(password_field) // .as_ref()
            .style(match app.selected_input {
                SelectedInput::Username => Style::default(),
                SelectedInput::Password => Style::default().fg(Color::Yellow),
            })
            .block(
                Block::default()
                .borders(Borders::ALL)
                .title("Password")
            )
            .wrap(Wrap { trim: true });
    f.render_widget(password_input, chunks[1]);

    match app.selected_input {
        SelectedInput::Username => {
            // Make the cursor visible and ask tui-rs to put it at the
            // specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[0].x + app.username_input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[0].y + 1,
            )
        },
        SelectedInput::Password => {
            // Make the cursor visible and ask tui-rs to put it at the
            // specified coordinates after rendering
            f.set_cursor(
                // Put cursor past the end of the input text
                chunks[1].x + app.password_input.width() as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1,
            )
        },
    }
}