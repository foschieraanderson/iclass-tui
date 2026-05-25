use ratatui::{
    layout::{
        Alignment,
        Constraint,
        Direction,
        Layout,
        Rect,
    },
    style::{
        Modifier,
        Style,
    },
    text::{
        Line,
        Span,
    },
    widgets::Paragraph,
    Frame,
};

use crate::{
    app::state::AppState,
    ui::theme,
};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);

    let bg = Style::default()
        .bg(theme::HEADER_BG)
        .fg(theme::HEADER_FG);

    // left: app name
    let left = Paragraph::new(
        Line::from(vec![
            Span::styled(
                " iClass",
                bg.fg(theme::BORDER_FOCUSED)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    )
    .style(bg);

    frame.render_widget(left, cols[0]);

    // right: user info
    let right_content = if let Some(ref session) = state.session {

        let role_color = match session.role.as_str() {
            "admin"   => theme::ROLE_ADMIN,
            "teacher" => theme::ROLE_TEACHER,
            _         => theme::ROLE_STUDENT,
        };

        Line::from(vec![
            Span::styled(
                session.email.clone(),
                bg.fg(theme::HEADER_FG),
            ),
            Span::styled(
                "  ·  ",
                bg.fg(theme::KEY_LABEL),
            ),
            Span::styled(
                session.role.clone(),
                bg.fg(role_color),
            ),
            Span::raw(" "),
        ])

    } else {
        Line::from("")
    };

    let right = Paragraph::new(right_content)
        .style(bg)
        .alignment(Alignment::Right);

    frame.render_widget(right, cols[1]);
}
