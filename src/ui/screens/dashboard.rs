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
            "Usuários: {:?}",
            state.users,
        )),
    ];

    let widget = Paragraph::new(text)
        .block(
            Block::default()
                .title("Dashboard")
                .borders(Borders::ALL),
        );

    frame.render_widget(widget, area);
}
