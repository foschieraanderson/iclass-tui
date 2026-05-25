use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::app::state::AppState;

pub fn render(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let text = vec![
        Line::from(format!(
            "Tarefas: {:?}",
            state.tasks,
        )),
    ];

    let widget = Paragraph::new(text)
        .block(
            Block::default()
                .title("Tarefas")
                .borders(Borders::ALL),
        );

    frame.render_widget(widget, area);
}
