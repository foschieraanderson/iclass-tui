use ratatui::{
    layout::Rect,
    style::{
        Color,
        Style,
    },
    text::{
        Line,
        Span,
    },
    widgets::Paragraph,
    Frame,
};

pub fn render(
    frame: &mut Frame,
    area: Rect,
) {

    let shortcuts = vec![
        Span::styled(
            " ↑↓",
            Style::default().fg(Color::Yellow),
        ),
        Span::styled(
            " navegar",
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            "   l",
            Style::default().fg(Color::Yellow),
        ),
        Span::styled(
            " logout",
            Style::default().fg(Color::DarkGray),
        ),
        Span::styled(
            "   q",
            Style::default().fg(Color::Yellow),
        ),
        Span::styled(
            " sair",
            Style::default().fg(Color::DarkGray),
        ),
    ];

    let widget = Paragraph::new(Line::from(shortcuts))
        .style(Style::default().bg(Color::Reset));

    frame.render_widget(widget, area);
}
