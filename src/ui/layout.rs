use ratatui::{
    layout::*,
    Frame,
};

// Retorna [header, sidebar, content, footer]
pub fn layout(
    frame: &Frame,
) -> Vec<Rect> {

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(frame.area());

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(30),
            Constraint::Min(1),
        ])
        .split(rows[1]);

    vec![rows[0], cols[0], cols[1], rows[2]]
}
