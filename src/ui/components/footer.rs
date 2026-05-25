use ratatui::{
    layout::Rect,
    style::Style,
    text::{
        Line,
        Span,
    },
    widgets::Paragraph,
    Frame,
};

use crate::{
    app::{
        routes::Route,
        state::{
            AppState,
            UserModal,
        },
    },
    ui::theme,
};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let spans = match state.route {

        Route::Users => {

            if state.user_modal != UserModal::None {

                vec![
                    key("Tab"), label(" próximo"),
                    key("   Espaço"), label(" perfil"),
                    key("   Enter"), label(" salvar"),
                    key("   Esc"), label(" cancelar"),
                ]

            } else {

                vec![
                    key("←→"), label(" foco"),
                    key("   ↑↓"), label(" navegar"),
                    key("   a"), label(" adicionar"),
                    key("   e"), label(" editar"),
                    key("   d"), label(" remover"),
                    key("   l"), label(" logout"),
                    key("   q"), label(" sair"),
                ]
            }
        }

        _ => {
            vec![
                key("↑↓"), label(" navegar"),
                key("   l"), label(" logout"),
                key("   q"), label(" sair"),
            ]
        }
    };

    let widget = Paragraph::new(Line::from(spans))
        .style(Style::default().bg(theme::HEADER_BG));

    frame.render_widget(widget, area);
}

fn key(text: &'static str) -> Span<'static> {
    Span::styled(text, Style::default().fg(theme::KEY_HINT))
}

fn label(text: &'static str) -> Span<'static> {
    Span::styled(text, Style::default().fg(theme::KEY_LABEL))
}
