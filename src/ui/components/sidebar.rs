use ratatui::{
    prelude::*,
    widgets::*,
};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    selected: usize,
) {

    let items = vec![
        ListItem::new("Dashboard"),
        ListItem::new("Usuários"),
        ListItem::new("Turmas"),
        ListItem::new("Tarefas"),
    ];

    let list = List::new(items)
        .highlight_symbol("▶ ")
        .highlight_style(
            Style::default()
                .fg(Color::Yellow),
        )
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
