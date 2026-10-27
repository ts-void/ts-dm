use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, Focus};

const ACCENT: Color = Color::Green;
const MUTED: Color = Color::DarkGray;
const TEXT: Color = Color::Gray;

fn field_style(focus: Focus, field: Focus) -> Style {
    if focus == field {
        Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(TEXT)
    }
}

pub fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(12),
            Constraint::Min(1),
        ])
        .split(area);

    let center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(vertical[1]);

    let username = Line::from(vec![
        Span::styled("Username: ", Style::default().fg(MUTED)),
        Span::styled(
            format!("[ {:<20} ]", app.username),
            field_style(app.focus, Focus::Username),
        ),
    ]);

    let masked_password = "*".repeat(app.password.chars().count());

    let password = Line::from(vec![
        Span::styled("Password: ", Style::default().fg(MUTED)),
        Span::styled(
            format!("[ {:<20} ]", masked_password),
            field_style(app.focus, Focus::Password),
        ),
    ]);

    let session_name = app
        .sessions
        .get(app.session_index)
        .map(|session| session.name.as_str())
        .unwrap_or("No sessions found");

    let session = Line::from(vec![
        Span::styled("Session:  ", Style::default().fg(MUTED)),
        Span::styled(
            format!("< {} >", session_name),
            field_style(app.focus, Focus::Session),
        ),
    ]);

    let login = Line::from(vec![
        Span::styled("[ Login ]", field_style(app.focus, Focus::Login)),
        Span::styled("    [ Esc: Exit ]", Style::default().fg(MUTED)),
    ]);

    let content = vec![
        Line::from(""),
        username,
        password,
        Line::from(""),
        session,
        Line::from(""),
        login,
        Line::from(Span::styled(
            &app.status,
            Style::default().fg(Color::Yellow),
        )),
    ];

    let widget = Paragraph::new(content).alignment(Alignment::Center).block(
        Block::default()
            .title(" void-greeter ")
            .title_style(Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(MUTED)),
    );

    frame.render_widget(widget, center[1]);
}
