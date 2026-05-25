use ratatui::{
    prelude::*,
    widgets::*,
};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    selected: usize,
    role: &str,
    focused: bool,
) {

    let items: Vec<ListItem> = if role == "admin" {
        vec![
            ListItem::new("Dashboard"),
            ListItem::new("Usuários"),
            ListItem::new("Turmas"),
            ListItem::new("Tarefas"),
        ]
    } else {
        vec![
            ListItem::new("Dashboard"),
            ListItem::new("Turmas"),
            ListItem::new("Tarefas"),
        ]
    };

    let highlight_style = if focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let list = List::new(items)
        .highlight_symbol("▶ ")
        .highlight_style(highlight_style)
        .block(
            Block::default()
                .title("Menu")
                .borders(Borders::ALL),
        );

    let mut state = ListState::default();

    state.select(Some(selected));

    frame.render_stateful_widget(
        list,
        area,
        &mut state,
    );
}
