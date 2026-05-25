use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::ui::theme;

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

    let border_style = if focused {
        Style::default().fg(theme::BORDER_FOCUSED)
    } else {
        Style::default().fg(theme::BORDER_INACTIVE)
    };

    let highlight_style = if focused {
        Style::default()
            .bg(theme::LIST_SELECTED_BG)
            .fg(theme::LIST_SELECTED_FG)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme::BORDER_INACTIVE)
    };

    let list = List::new(items)
        .highlight_symbol("▶ ")
        .highlight_style(highlight_style)
        .block(
            Block::default()
                .title("Menu")
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    let mut state = ListState::default();

    state.select(Some(selected));

    frame.render_stateful_widget(
        list,
        area,
        &mut state,
    );
}
