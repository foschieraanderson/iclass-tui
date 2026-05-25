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
        Modifier,
        Style,
    },
    text::{
        Line,
        Span,
    },
    widgets::{
        Block,
        Borders,
        Clear,
        List,
        ListItem,
        ListState,
        Paragraph,
    },
    Frame,
};

use crate::{
    app::{
        resources::Resource,
        state::{
            AppState,
            UserFormField,
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

    // -- admin gate --------------------------------------------

    let is_admin = state.session
        .as_ref()
        .map(|s| s.role == "admin")
        .unwrap_or(false);

    if !is_admin {

        let msg = Paragraph::new("Acesso restrito")
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme::ERROR))
            .block(
                Block::default()
                    .title("Usuários")
                    .borders(Borders::ALL),
            );

        frame.render_widget(msg, area);
        return;
    }

    // -- user list ---------------------------------------------

    render_list(frame, area, state);

    // -- floating modals (rendered on top) ---------------------

    match state.user_modal {

        UserModal::Add | UserModal::Edit => {
            render_form_modal(frame, frame.area(), state);
        }

        UserModal::ConfirmDelete => {
            render_confirm_modal(frame, frame.area(), state);
        }

        UserModal::None => {}
    }
}

// ---- list ----------------------------------------------------

fn render_list(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    match &state.users {

        Resource::Idle => {

            let msg = Paragraph::new("Nenhum dado carregado.")
                .block(
                    Block::default()
                        .title("Usuários")
                        .borders(Borders::ALL),
                );

            frame.render_widget(msg, area);
        }

        Resource::Loading => {

            let msg = Paragraph::new("Carregando...")
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .title("Usuários")
                        .borders(Borders::ALL),
                );

            frame.render_widget(msg, area);
        }

        Resource::Error(e) => {

            let msg = Paragraph::new(e.clone())
                .style(Style::default().fg(theme::ERROR))
                .block(
                    Block::default()
                        .title("Usuários")
                        .borders(Borders::ALL),
                );

            frame.render_widget(msg, area);
        }

        Resource::Success(users) => {

            let items: Vec<ListItem> = users
                .iter()
                .map(|u| {

                    let role_style = match u.role.as_str() {
                        "admin"   => Style::default().fg(Color::Red),
                        "teacher" => Style::default().fg(Color::Yellow),
                        _         => Style::default().fg(Color::Gray),
                    };

                    ListItem::new(Line::from(vec![
                        Span::styled(
                            format!("{:<30}", u.name),
                            Style::default(),
                        ),
                        Span::styled(
                            format!("{:<35}", u.email),
                            Style::default().fg(Color::DarkGray),
                        ),
                        Span::styled(
                            u.role.clone(),
                            role_style,
                        ),
                    ]))
                })
                .collect();

            let list = List::new(items)
                .highlight_symbol("▶ ")
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .add_modifier(Modifier::BOLD),
                )
                .block(
                    Block::default()
                        .title(format!("Usuários ({})", users.len()))
                        .borders(Borders::ALL),
                );

            let mut list_state = ListState::default();
            list_state.select(Some(state.selected_user_index));

            frame.render_stateful_widget(list, area, &mut list_state);
        }
    }
}

// ---- add / edit modal ----------------------------------------

fn render_form_modal(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(70),
            Constraint::Percentage(15),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(vertical[1]);

    let modal_area = horizontal[1];

    frame.render_widget(Clear, modal_area);

    let title = if state.user_modal == UserModal::Add {
        "Adicionar Usuário"
    } else {
        "Editar Usuário"
    };

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),  // título
            Constraint::Length(3),  // nome
            Constraint::Length(3),  // email
            Constraint::Length(3),  // senha
            Constraint::Length(3),  // perfil
            Constraint::Length(1),  // espaço
            Constraint::Length(1),  // help
        ])
        .split(modal_area);

    // outer block (borda do modal)
    let outer = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    frame.render_widget(outer, modal_area);

    // nome
    render_form_field(
        frame,
        sections[1],
        "Nome",
        &state.user_form.name,
        false,
        state.user_form.active_field == UserFormField::Name,
    );

    // email
    render_form_field(
        frame,
        sections[2],
        "Email",
        &state.user_form.email,
        false,
        state.user_form.active_field == UserFormField::Email,
    );

    // senha
    render_form_field(
        frame,
        sections[3],
        if state.user_modal == UserModal::Edit { "Senha (vazio = sem alterar)" } else { "Senha" },
        &state.user_form.password,
        true,
        state.user_form.active_field == UserFormField::Password,
    );

    // perfil (cicla com Espaço)
    render_role_field(
        frame,
        sections[4],
        &state.user_form.role,
        state.user_form.active_field == UserFormField::Role,
    );

    // help
    let help = Paragraph::new(
        "Tab: próximo   Espaço: perfil   Enter: salvar   Esc: cancelar",
    )
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(help, sections[6]);
}

fn render_form_field(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    mask: bool,
    active: bool,
) {

    let border_style = if active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let text = if mask {
        "*".repeat(value.len())
    } else {
        value.to_string()
    };

    let widget = Paragraph::new(text)
        .block(
            Block::default()
                .title(label)
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    frame.render_widget(widget, area);
}

fn render_role_field(
    frame: &mut Frame,
    area: Rect,
    role: &str,
    active: bool,
) {

    let border_style = if active {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let role_color = match role {
        "admin"   => Color::Red,
        "teacher" => Color::Yellow,
        _         => Color::Gray,
    };

    let content = Line::from(vec![
        Span::styled("< ", Style::default().fg(Color::DarkGray)),
        Span::styled(role, Style::default().fg(role_color)),
        Span::styled(" >", Style::default().fg(Color::DarkGray)),
    ]);

    let widget = Paragraph::new(content)
        .block(
            Block::default()
                .title("Perfil")
                .borders(Borders::ALL)
                .border_style(border_style),
        );

    frame.render_widget(widget, area);
}

// ---- confirm delete modal ------------------------------------

fn render_confirm_modal(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(38),
            Constraint::Percentage(24),
            Constraint::Percentage(38),
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

    let modal_area = horizontal[1];

    frame.render_widget(Clear, modal_area);

    let user_name = if let Resource::Success(ref list) = state.users {
        list.get(state.selected_user_index)
            .map(|u| u.name.clone())
            .unwrap_or_default()
    } else {
        String::new()
    };

    let content = vec![
        Line::from(""),
        Line::from(
            Span::styled(
                format!("Remover \"{}\"?", user_name),
                Style::default().fg(Color::White),
            ),
        ),
        Line::from(""),
        Line::from(
            Span::styled(
                "Enter: confirmar   Esc: cancelar",
                Style::default().fg(Color::DarkGray),
            ),
        ),
    ];

    let widget = Paragraph::new(content)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("Confirmar exclusão")
                .borders(Borders::ALL)
                .border_style(
                    Style::default().fg(theme::ERROR),
                ),
        );

    frame.render_widget(widget, modal_area);
}
