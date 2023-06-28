use std::{error::Error, io, thread, time::Duration};
use crossterm::{
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode
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
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

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
    // Which input box is currently in use
    selected_input: SelectedInput,
}

impl Default for App {
    fn default() -> App {
        App {
            username_input: String::new(),
            password_input: String::new(),
            selected_input: SelectedInput::Username,
        }
    }
}

impl App {
    fn invert_selected_input(&mut self) {
        match self.selected_input {
            SelectedInput::Username => {
                self.selected_input = SelectedInput::Password;
            },
            SelectedInput::Password => {
                self.selected_input = SelectedInput::Username;
            },
        }
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
    let mut app = App::default();
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
        println!("ERROR.LOGIN: {:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> 
    io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => {
                    // Attempt to log in
                    // Return username and password to the main file
                    // For now, just exit
                    // return Ok(());
                },
                KeyCode::Backspace => {
                    match app.selected_input {
                        SelectedInput::Username => 
                            app.username_input.pop(),
                        SelectedInput::Password => 
                            app.password_input.pop(),
                    };
                },
                KeyCode::Tab => {
                    // If Tab is pressed, switch fields
                    // match app.selected_input {
                    //     SelectedInput::Username => 
                    //         app.selected_input = SelectedInput::Password,
                    //     SelectedInput::Password => 
                    //         app.selected_input = SelectedInput::Username,
                    // };
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
                            app.password_input.push(c);
                            continue;
                        },
                    };
                },
                _ => {},
            }
        }
        thread::sleep(Duration::from_millis(50));
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                // Constraint::Length(1),
                // Constraint::Length(3),
                // Constraint::Min(1),
                Constraint::Length(3),
                Constraint::Length(3),
                // The Min here is required in order for the password box
                // not to auto-extend to the bottom
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    // let (msg, style) = match app.selected_input {
    //     SelectedInput::Username => (
    //         vec![
    //             Span::raw("Press "),
    //             Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
    //             Span::raw(" to exit, "),
    //             Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
    //             Span::raw(" to start editing."),
    //         ],
    //         Style::default().add_modifier(Modifier::RAPID_BLINK),
    //     ),
    //     SelectedInput::Password => (
    //         vec![
    //             Span::raw("Press "),
    //             Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
    //             Span::raw(" to stop editing, "),
    //             Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
    //             Span::raw(" to record the message"),
    //         ],
    //         Style::default(),
    //     ),
    // };

    // let mut text = Text::from(Spans::from(msg));
    // text.patch_style(style);
    // let help_message = Paragraph::new(text);
    // f.render_widget(help_message, chunks[0]);

    let username_input =
        Paragraph::new(app.username_input.as_ref())
        .style(match app.selected_input {
            SelectedInput::Username => Style::default().fg(Color::Yellow),
            SelectedInput::Password => Style::default(),
        })
        .block(
            Block::default()
            .borders(Borders::ALL)
            .title("Username")
            // .title_alignment(Alignment::Center)
        );
    f.render_widget(username_input, chunks[0]);

    let password_input =
        Paragraph::new(app.password_input.as_ref())
        .style(match app.selected_input {
            SelectedInput::Username => Style::default(),
            SelectedInput::Password => Style::default().fg(Color::Yellow),
        })
        .block(
            Block::default()
            .borders(Borders::ALL)
            .title("Password")
            // .title_alignment(Alignment::Center)
        );
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

    // let messages: Vec<ListItem> = app
    //     .messages
    //     .iter()
    //     .enumerate()
    //     .map(|(i, m)| {
    //         let content = vec![Spans::from(Span::raw(format!("{}: {}", i, m)))];
    //         ListItem::new(content)
    //     })
    //     .collect();
    // let messages =
    //     List::new(messages).block(Block::default().borders(Borders::ALL).title("Messages"));
    // f.render_widget(messages, chunks[2]);
}