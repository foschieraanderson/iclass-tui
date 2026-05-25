use ratatui::{
    layout::{
        Alignment,
        Constraint,
        Direction,
        Layout,
        Rect,
    },
    style::{
        Color,
        Style,
    },
    text::{
        Line,
        Span,
    },
    widgets::{
        Block,
        Borders,
        Paragraph,
    },
    Frame,
};

use crate::{
    app::state::{
        AppState,
        LoginField,
    },
    ui::theme,
};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(30),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(vertical[1]);

    let form_area = horizontal[1];

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .split(form_area);

    render_title(frame, sections[0]);
    render_email(frame, sections[1], state);
    render_password(frame, sections[2], state);
    render_status(frame, sections[3], state);
    render_help(frame, sections[4]);
}

fn render_title(
    frame: &mut Frame,
    area: Rect,
) {

    let title = Paragraph::new("iClass — Login")
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL),
        );

    frame.render_widget(title, area);
}

fn render_email(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let is_active =
        state.login_form.active_field == LoginField::Email;

    let border_style = if is_active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let text_style = if is_active {
        Style::default().fg(Color::White)
    } else {
        Style::default()
    };

    let widget = Paragraph::new(
        Span::styled(
            state.login_form.email.clone(),
            text_style,
        ),
    )
    .block(
        Block::default()
            .title("Email")
            .borders(Borders::ALL)
            .border_style(border_style),
    );

    frame.render_widget(widget, area);
}

fn render_password(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let is_active =
        state.login_form.active_field == LoginField::Password;

    let border_style = if is_active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let text_style = if is_active {
        Style::default().fg(Color::White)
    } else {
        Style::default()
    };

    let masked: String =
        "*".repeat(state.login_form.password.len());

    let widget = Paragraph::new(
        Span::styled(masked, text_style),
    )
    .block(
        Block::default()
            .title("Senha")
            .borders(Borders::ALL)
            .border_style(border_style),
    );

    frame.render_widget(widget, area);
}

fn render_status(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let text = if state.loading {

        Line::from(
            Span::styled(
                "Autenticando...",
                Style::default().fg(Color::Yellow),
            ),
        )

    } else if let Some(ref err) = state.error {

        Line::from(
            Span::styled(
                err.clone(),
                Style::default().fg(theme::ERROR),
            ),
        )

    } else {

        Line::from("")
    };

    let widget = Paragraph::new(text)
        .alignment(Alignment::Center);

    frame.render_widget(widget, area);
}

fn render_help(
    frame: &mut Frame,
    area: Rect,
) {

    let help = Paragraph::new(
        "Tab: alternar campo  Enter: entrar  Ctrl+C: sair",
    )
    .alignment(Alignment::Center)
    .style(
        Style::default().fg(Color::DarkGray),
    );

    frame.render_widget(help, area);
}
