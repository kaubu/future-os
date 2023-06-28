use std::{error::Error, io, thread, time::Duration};

use crate::{VERSION, api};

use crossterm::{
    event::{
        self,
        DisableMouseCapture,
        EnableMouseCapture,
        Event,
        KeyCode, poll, KeyEventKind, ModifierKeyCode
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
    layout::{Constraint, Direction, Layout, Alignment, Rect},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph, Wrap, Clear},
    Frame, Terminal, terminal, text::{Line, Span},
};
use unicode_width::UnicodeWidthStr;

const CENSOR_PASSWORD: bool = true;
const DEFAULT_USERNAME: &str = "admin";
const DEFAULT_PASSWORD: &str = "foobar";

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

    // Pop up information Whether the popup shows or not
    show_popup: bool,
    // The popup's title
    popup_title: String,
    // The popup's content
    popup_content: String,
}

impl Default for App {
    fn default() -> App {
        App {
            username_input: String::new(),
            password_input: String::new(),
            censored_input: String::new(),
            selected_input: SelectedInput::Username,
            show_popup: false,
            popup_title: String::new(),
            popup_content: String::new(),
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

        if app.show_popup {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => app.show_popup = false,
                        _ => {}
                    }
                }
            }

            continue;
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Enter => {
                        // Attempt to log in Return username and password to
                        // the main file For now, just exit return Ok(());
                        let correct_login = authenticate(
                            app.username_input.to_string(),
                            app.password_input.to_string(),
                        );

                        if correct_login {
                            app.popup_title =
                                String::from("Login Alert");
                            app.popup_content =
                                String::from("Successfully logged in!");
                            app.show_popup = true;
                        } else {
                            app.popup_title =
                                String::from("Login Alert");
                            app.popup_content =
                                String::from("Failed to log in! \
Please check your username and password!");
                            app.show_popup = true;
                        }
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
                    KeyCode::Esc => {
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
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(5) // space on the side
        .vertical_margin(2)
        .constraints(
            [
                Constraint::Length(3), // 0
                Constraint::Length(3), // 1
                // The Min here is required in order for the password box not
                // to auto-extend to the bottom
                Constraint::Min(1), // 2
                Constraint::Length(1), // 3
            ]
            .as_ref(),
        )
        .split(size);

    let border = Block::default()
        .title(api::get_window_title("Login"))
        .borders(Borders::ALL);
    f.render_widget(border, size);

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

    let help_text = vec![
        // switch fields: <tab> | login: <enter> | logout: <q>
        Line::from(vec![
            Span::raw("switch fields: "),
            Span::styled("<tab>", Style::default()
                .add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::raw("login: "),
            Span::styled("<enter>", Style::default()
                .add_modifier(Modifier::BOLD)),
            Span::raw(" | "),
            Span::raw("logout: "),
            Span::styled("<esc>", Style::default()
                .add_modifier(Modifier::BOLD)),
        ]),
    ];
    let help_menu = Paragraph::new(help_text)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
    f.render_widget(help_menu, chunks[3]);

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

    if app.show_popup {
        let area = centered_rect(60, 14, size);
        let text_area =
            centered_rect(58, 10, size);

        let paragraph = Paragraph::new(Span::styled(
            app.popup_content.to_string(),
            Style::default().add_modifier(Modifier::SLOW_BLINK),
        ))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
         // chunks[0]

        let block = Block::default()
            .title(app.popup_title.to_string())
            .borders(Borders::ALL);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(block, area);
        f.render_widget(paragraph, text_area);
    }
}

// helper function to create a centered rect using up certain percentage of
// the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn authenticate(username: String, password: String) -> bool {
    if username == DEFAULT_USERNAME && password == DEFAULT_PASSWORD {
        return true;
    }

    return false;
}