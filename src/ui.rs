use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(frame: &mut Frame) {
    let area = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(35),
            Constraint::Length(9),
            Constraint::Percentage(35),
        ])
        .split(area);

    let center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(vertical[1]);

    let login = Paragraph::new(
        "VOID LINUX\n\n\
         Username:  [                 ]\n\
         Password:  [                 ]\n\n\
         [ Login ]     [ Session ]",
    )
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .title(" void-greeter ")
            .borders(Borders::ALL),
    );

    frame.render_widget(login, center[1]);
}
