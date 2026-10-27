use crate::session::{Session, discover_sessions};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    Username,
    Password,
    Session,
    Login,
}

pub struct App {
    pub username: String,
    pub password: String,
    pub sessions: Vec<Session>,
    pub session_index: usize,
    pub focus: Focus,
    pub status: String,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        let sessions = discover_sessions();

        let status = if sessions.is_empty() {
            "No Wayland sessions found.".to_string()
        } else {
            String::new()
        };

        Self {
            username: String::new(),
            password: String::new(),
            sessions,
            session_index: 0,
            focus: Focus::Username,
            status,
            should_quit: false,
        }
    }
}

impl App {
    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;
            return;
        }

        match key.code {
            KeyCode::Esc => self.should_quit = true,

            KeyCode::Tab => self.next_focus(),
            KeyCode::BackTab => self.previous_focus(),

            KeyCode::Up | KeyCode::Down if self.focus == Focus::Session => {
                let delta = if key.code == KeyCode::Down { 1 } else { -1 };
                self.change_session(delta);
            }

            KeyCode::Left | KeyCode::Right if self.focus == Focus::Session => {
                let delta = if key.code == KeyCode::Right { 1 } else { -1 };
                self.change_session(delta);
            }

            KeyCode::Enter => self.activate(),

            KeyCode::Backspace => match self.focus {
                Focus::Username => {
                    self.username.pop();
                }
                Focus::Password => {
                    self.password.pop();
                }
                _ => {}
            },

            KeyCode::Char(c)
                if !key.modifiers.contains(KeyModifiers::CONTROL)
                    && !key.modifiers.contains(KeyModifiers::ALT) =>
            {
                match self.focus {
                    Focus::Username => self.username.push(c),
                    Focus::Password => self.password.push(c),
                    _ => {}
                }
            }

            _ => {}
        }
    }

    fn next_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Username => Focus::Password,
            Focus::Password => Focus::Session,
            Focus::Session => Focus::Login,
            Focus::Login => Focus::Username,
        };
    }

    fn previous_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Username => Focus::Login,
            Focus::Password => Focus::Username,
            Focus::Session => Focus::Password,
            Focus::Login => Focus::Session,
        };
    }

    fn change_session(&mut self, delta: isize) {
        let len = self.sessions.len();

        if len == 0 {
            return;
        }

        self.session_index =
            (self.session_index as isize + delta).rem_euclid(len as isize) as usize;
    }

    fn activate(&mut self) {
        match self.focus {
            Focus::Username => {
                self.focus = Focus::Password;
            }
            Focus::Password => {
                self.focus = Focus::Session;
            }
            Focus::Session => {
                self.focus = Focus::Login;
            }
            Focus::Login => {
                self.status = if self.username.trim().is_empty() {
                    "Username is required.".to_string()
                } else if self.password.is_empty() {
                    "Password is required.".to_string()
                } else {
                    "Authentication is not implemented yet.".to_string()
                };
            }
        }
    }
}
