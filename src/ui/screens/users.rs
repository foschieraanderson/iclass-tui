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
        focus::Focus,
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
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme::BORDER_INACTIVE)),
            );

        frame.render_widget(msg, area);
        return;
    }

    // -- user list + detail panel ------------------------------

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(8)])
        .split(area);

    render_list(frame, chunks[0], state);
    render_detail(frame, chunks[1], state);

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

    let content_focused = state.focus == Focus::Content;

    let border_style = if content_focused {
        Style::default().fg(theme::BORDER_FOCUSED)
    } else {
        Style::default().fg(theme::BORDER_INACTIVE)
    };

    match &state.users {

        Resource::Idle => {

            let msg = Paragraph::new("Nenhum dado carregado.")
                .block(
                    Block::default()
                        .title("Usuários")
                        .borders(Borders::ALL)
                        .border_style(border_style),
                );

            frame.render_widget(msg, area);
        }

        Resource::Loading => {

            let msg = Paragraph::new("Carregando...")
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .title("Usuários")
                        .borders(Borders::ALL)
                        .border_style(border_style),
                );

            frame.render_widget(msg, area);
        }

        Resource::Error(e) => {

            let msg = Paragraph::new(e.clone())
                .style(Style::default().fg(theme::ERROR))
                .block(
                    Block::default()
                        .title("Usuários")
                        .borders(Borders::ALL)
                        .border_style(border_style),
                );

            frame.render_widget(msg, area);
        }

        Resource::Success(users) => {

            let items: Vec<ListItem> = users
                .iter()
                .map(|u| {

                    let role_style = match u.role.as_str() {
                        "admin"   => Style::default().fg(theme::ROLE_ADMIN),
                        "teacher" => Style::default().fg(theme::ROLE_TEACHER),
                        _         => Style::default().fg(theme::ROLE_STUDENT),
                    };

                    ListItem::new(Line::from(vec![
                        Span::styled(
                            format!("{:<30}", u.name),
                            Style::default(),
                        ),
                        Span::styled(
                            format!("{:<35}", u.email),
                            Style::default().fg(theme::KEY_LABEL),
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
                        .bg(theme::LIST_SELECTED_BG)
                        .fg(theme::LIST_SELECTED_FG)
                        .add_modifier(Modifier::BOLD),
                )
                .block(
                    Block::default()
                        .title(format!("Usuários ({})", users.len()))
                        .borders(Borders::ALL)
                        .border_style(border_style),
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

    let is_add = state.user_modal == UserModal::Add;

    let title = if is_add { "Adicionar Usuário" } else { "Editar Usuário" };

    // Add: Nome, Email, Senha, Perfil (7 seções)
    // Edit: Nome, Email, Perfil (6 seções — sem Senha)
    let constraints: Vec<Constraint> = if is_add {
        vec![
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
        ]
    } else {
        vec![
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(1),
        ]
    };

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(modal_area);

    let outer = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER_FOCUSED));

    frame.render_widget(outer, modal_area);

    render_form_field(
        frame,
        sections[1],
        "Nome",
        &state.user_form.name,
        false,
        state.user_form.active_field == UserFormField::Name,
    );

    render_form_field(
        frame,
        sections[2],
        "Email",
        &state.user_form.email,
        false,
        state.user_form.active_field == UserFormField::Email,
    );

    let help = Paragraph::new(
        "Tab: próximo   Espaço: perfil   Enter: salvar   Esc: cancelar",
    )
    .alignment(Alignment::Center)
    .style(Style::default().fg(theme::KEY_LABEL));

    if is_add {
        render_form_field(
            frame,
            sections[3],
            "Senha",
            &state.user_form.password,
            true,
            state.user_form.active_field == UserFormField::Password,
        );
        render_role_field(
            frame,
            sections[4],
            &state.user_form.role,
            state.user_form.active_field == UserFormField::Role,
        );
        frame.render_widget(help, sections[6]);
    } else {
        render_role_field(
            frame,
            sections[3],
            &state.user_form.role,
            state.user_form.active_field == UserFormField::Role,
        );
        frame.render_widget(help, sections[5]);
    }
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
        Style::default().fg(theme::BORDER_FOCUSED)
    } else {
        Style::default().fg(theme::BORDER_INACTIVE)
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
        Style::default().fg(theme::BORDER_FOCUSED)
    } else {
        Style::default().fg(theme::BORDER_INACTIVE)
    };

    let role_color = match role {
        "admin"   => theme::ROLE_ADMIN,
        "teacher" => theme::ROLE_TEACHER,
        _         => theme::ROLE_STUDENT,
    };

    let content = Line::from(vec![
        Span::styled("< ", Style::default().fg(theme::KEY_LABEL)),
        Span::styled(role, Style::default().fg(role_color)),
        Span::styled(" >", Style::default().fg(theme::KEY_LABEL)),
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

// ---- detail panel --------------------------------------------

fn render_detail(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let block = Block::default()
        .title("Detalhes")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER_INACTIVE));

    match &state.users {

        Resource::Success(users) => {

            if let Some(user) = users.get(state.selected_user_index) {

                let role_color = match user.role.as_str() {
                    "admin"   => theme::ROLE_ADMIN,
                    "teacher" => theme::ROLE_TEACHER,
                    _         => theme::ROLE_STUDENT,
                };

                let lines = vec![
                    Line::from(vec![
                        Span::styled("ID:     ", Style::default().fg(theme::KEY_LABEL)),
                        Span::raw(user.id.clone()),
                    ]),
                    Line::from(vec![
                        Span::styled("Nome:   ", Style::default().fg(theme::KEY_LABEL)),
                        Span::styled(user.name.clone(), Style::default().fg(theme::HEADER_FG)),
                    ]),
                    Line::from(vec![
                        Span::styled("Email:  ", Style::default().fg(theme::KEY_LABEL)),
                        Span::raw(user.email.clone()),
                    ]),
                    Line::from(vec![
                        Span::styled("Perfil: ", Style::default().fg(theme::KEY_LABEL)),
                        Span::styled(user.role.clone(), Style::default().fg(role_color)),
                    ]),
                ];

                frame.render_widget(Paragraph::new(lines).block(block), area);

            } else {

                frame.render_widget(
                    Paragraph::new("—").block(block),
                    area,
                );
            }
        }

        _ => {
            frame.render_widget(
                Paragraph::new("Selecione um usuário para ver os detalhes.")
                    .style(Style::default().fg(theme::KEY_LABEL))
                    .block(block),
                area,
            );
        }
    }
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
                Style::default().fg(theme::HEADER_FG),
            ),
        ),
        Line::from(""),
        Line::from(
            Span::styled(
                "Enter: confirmar   Esc: cancelar",
                Style::default().fg(theme::KEY_LABEL),
            ),
        ),
    ];

    let widget = Paragraph::new(content)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .title("Confirmar exclusão")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme::ERROR)),
        );

    frame.render_widget(widget, modal_area);
}
