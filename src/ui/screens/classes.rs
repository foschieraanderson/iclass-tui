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
            ClassFormField,
            ClassModal,
            PICKER_VISIBLE_ROWS,
        },
    },
    models::user::User,
    ui::theme,
};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let is_admin = state.session
        .as_ref()
        .map(|s| s.role == "admin")
        .unwrap_or(false);

    render_list(frame, area, state, is_admin);

    match state.class_modal {

        ClassModal::Add | ClassModal::Edit => {
            render_form_modal(frame, frame.area(), state);
        }

        ClassModal::ConfirmDelete => {
            render_confirm_modal(frame, frame.area(), state);
        }

        ClassModal::None => {}
    }
}

// ---- lista de turmas -----------------------------------------

fn render_list(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    is_admin: bool,
) {

    let content_focused = state.focus == Focus::Content;

    let border_style = if content_focused {
        Style::default().fg(theme::BORDER_FOCUSED)
    } else {
        Style::default().fg(theme::BORDER_INACTIVE)
    };

    match &state.classes {

        Resource::Idle => {
            frame.render_widget(
                Paragraph::new("Nenhum dado carregado.")
                    .block(list_block("Turmas", border_style)),
                area,
            );
        }

        Resource::Loading => {
            frame.render_widget(
                Paragraph::new("Carregando...")
                    .style(Style::default().fg(Color::Yellow))
                    .block(list_block("Turmas", border_style)),
                area,
            );
        }

        Resource::Error(e) => {
            frame.render_widget(
                Paragraph::new(e.clone())
                    .style(Style::default().fg(theme::ERROR))
                    .block(list_block("Turmas", border_style)),
                area,
            );
        }

        Resource::Success(classes) => {

            if classes.is_empty() {

                let msg = if is_admin {
                    "Nenhuma turma cadastrada. Pressione 'a' para adicionar."
                } else {
                    "Nenhuma turma encontrada."
                };

                frame.render_widget(
                    Paragraph::new(msg)
                        .style(Style::default().fg(theme::KEY_LABEL))
                        .block(list_block("Turmas (0)", border_style)),
                    area,
                );
                return;
            }

            let items: Vec<ListItem> = classes
                .iter()
                .map(|c| {

                    let student_count = c.students.len();

                    ListItem::new(Line::from(vec![

                        Span::styled(
                            format!("{:<14}", c.code),
                            Style::default()
                                .fg(theme::BORDER_FOCUSED)
                                .add_modifier(Modifier::BOLD),
                        ),

                        Span::styled(
                            format!("{:<28}", c.teacher.name),
                            Style::default().fg(theme::HEADER_FG),
                        ),

                        Span::styled(
                            format!("{:<32}", c.teacher.email),
                            Style::default().fg(theme::KEY_LABEL),
                        ),

                        Span::styled(
                            format!(
                                "{} aluno{}",
                                student_count,
                                if student_count == 1 { "" } else { "s" },
                            ),
                            Style::default().fg(theme::ROLE_STUDENT),
                        ),
                    ]))
                })
                .collect();

            let title = format!(
                "Turmas ({})  [código · professor · alunos]",
                classes.len(),
            );

            let list = List::new(items)
                .highlight_symbol("▶ ")
                .highlight_style(
                    Style::default()
                        .bg(theme::LIST_SELECTED_BG)
                        .fg(theme::LIST_SELECTED_FG)
                        .add_modifier(Modifier::BOLD),
                )
                .block(list_block(&title, border_style));

            let mut list_state = ListState::default();
            list_state.select(Some(state.selected_class_index));

            frame.render_stateful_widget(list, area, &mut list_state);
        }
    }
}

fn list_block(title: &str, border_style: Style) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style)
}

// ---- modal Add/Edit ------------------------------------------

fn render_form_modal(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let picker_height = PICKER_VISIBLE_ROWS as u16 + 2; // borda + linhas visíveis

    // Modal height: padding(1) + period(3) + grade(3) + teacher(picker_h) + students(picker_h) + error(1) + help(1) + outer border(2)
    let min_height = 1 + 3 + 3 + picker_height + picker_height + 1 + 1 + 2;

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(3),
            Constraint::Min(min_height),
            Constraint::Percentage(10),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Percentage(5),
        ])
        .split(vertical[1]);

    let modal_area = horizontal[1];

    frame.render_widget(Clear, modal_area);

    let title = if state.class_modal == ClassModal::Add {
        "Adicionar Turma"
    } else {
        "Editar Turma"
    };

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),           // padding topo
            Constraint::Length(3),           // Período
            Constraint::Length(3),           // Série
            Constraint::Length(picker_height), // Teacher picker
            Constraint::Length(picker_height), // Student picker
            Constraint::Length(1),           // erro inline
            Constraint::Length(1),           // ajuda
        ])
        .split(modal_area);

    let outer = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER_FOCUSED));

    frame.render_widget(outer, modal_area);

    // Campos de texto
    render_text_field(
        frame,
        inner[1],
        "Período  (ex: 2026/1)",
        &state.class_form.period,
        state.class_form.active_field == ClassFormField::Period,
    );

    render_text_field(
        frame,
        inner[2],
        "Série  (ex: 3A)",
        &state.class_form.grade,
        state.class_form.active_field == ClassFormField::Grade,
    );

    // Pickers
    render_teacher_picker(frame, inner[3], state);
    render_student_picker(frame, inner[4], state);

    // Erro inline
    if let Some(ref err) = state.error {
        frame.render_widget(
            Paragraph::new(
                Span::styled(err.as_str(), Style::default().fg(theme::ERROR)),
            )
            .alignment(Alignment::Center),
            inner[5],
        );
    }

    // Ajuda
    frame.render_widget(
        Paragraph::new(
            "Tab: próximo campo   ↑/↓: navegar   Espaço: selecionar   Enter: salvar   Esc: cancelar",
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme::KEY_LABEL)),
        inner[6],
    );
}

fn render_text_field(
    frame: &mut Frame,
    area: Rect,
    label: &str,
    value: &str,
    active: bool,
) {

    let border_style = if active {
        Style::default().fg(theme::BORDER_FOCUSED)
    } else {
        Style::default().fg(theme::BORDER_INACTIVE)
    };

    frame.render_widget(
        Paragraph::new(value.to_string())
            .block(
                Block::default()
                    .title(label)
                    .borders(Borders::ALL)
                    .border_style(border_style),
            ),
        area,
    );
}

// ---- helpers de scroll ---------------------------------------

/// Sufixo de título indicando itens fora da janela visível.
/// Ex: `"  ↓8 mais"`, `"  ↑2 acima  ↓5 mais"` ou `""`.
fn scroll_indicator(scroll: usize, total: usize, visible: usize) -> String {
    let above = scroll;
    let below = total.saturating_sub(scroll + visible);
    match (above, below) {
        (0, 0) => String::new(),
        (0, b) => format!("  ↓{} mais", b),
        (a, 0) => format!("  ↑{} acima", a),
        (a, b) => format!("  ↑{} acima  ↓{} mais", a, b),
    }
}

fn picker_block(title: String, active: bool) -> Block<'static> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(if active {
            Style::default().fg(theme::BORDER_FOCUSED)
        } else {
            Style::default().fg(theme::BORDER_INACTIVE)
        })
}

// ---- teacher picker (single-select) --------------------------

fn render_teacher_picker(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {
    let active = state.class_form.active_field == ClassFormField::TeacherPicker;

    // Nome do professor atualmente selecionado (para exibir no título)
    let selected_name: Option<String> =
        if let (Some(idx), Resource::Success(teachers)) =
            (state.class_form.selected_teacher, &state.available_teachers)
        {
            teachers.get(idx).map(|u| u.name.clone())
        } else {
            None
        };

    match &state.available_teachers {

        Resource::Idle | Resource::Loading => {
            let block = picker_block("Professor".to_string(), active);
            frame.render_widget(
                Paragraph::new("Carregando...")
                    .style(Style::default().fg(Color::Yellow))
                    .block(block),
                area,
            );
        }

        Resource::Error(e) => {
            let block = picker_block("Professor".to_string(), active);
            frame.render_widget(
                Paragraph::new(e.clone())
                    .style(Style::default().fg(theme::ERROR))
                    .block(block),
                area,
            );
        }

        Resource::Success(teachers) => {

            if teachers.is_empty() {
                let block = picker_block("Professor".to_string(), active);
                frame.render_widget(
                    Paragraph::new("Nenhum professor encontrado.")
                        .style(Style::default().fg(theme::KEY_LABEL))
                        .block(block),
                    area,
                );
                return;
            }

            let cursor   = state.class_form.teacher_cursor;
            let scroll   = state.class_form.teacher_scroll;
            let selected = state.class_form.selected_teacher;
            let total    = teachers.len();

            let base = match selected_name {
                Some(ref name) => format!("Professor  ● {}", name),
                None           => "Professor  (nenhum selecionado)".to_string(),
            };
            let title = format!("{}{}", base, scroll_indicator(scroll, total, PICKER_VISIBLE_ROWS));
            let block = picker_block(title, active);

            let items: Vec<ListItem> = teachers
                .iter()
                .enumerate()
                .skip(scroll)
                .take(PICKER_VISIBLE_ROWS)
                .map(|(i, user)| teacher_item(i, user, selected))
                .collect();

            let list = List::new(items)
                .highlight_style(
                    Style::default()
                        .bg(theme::LIST_SELECTED_BG)
                        .fg(theme::LIST_SELECTED_FG),
                )
                .block(block);

            let mut list_state = ListState::default();
            if active {
                list_state.select(Some(cursor.saturating_sub(scroll)));
            }

            frame.render_stateful_widget(list, area, &mut list_state);
        }
    }
}

fn teacher_item(
    idx: usize,
    user: &User,
    selected: Option<usize>,
) -> ListItem<'static> {

    let is_selected = selected == Some(idx);

    let indicator = if is_selected {
        Span::styled("● ", Style::default().fg(Color::Green))
    } else {
        Span::styled("  ", Style::default())
    };

    let name = Span::styled(
        format!("{:<28}", user.name.clone()),
        if is_selected {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(theme::HEADER_FG)
        },
    );

    let email = Span::styled(
        user.email.clone(),
        Style::default().fg(theme::KEY_LABEL),
    );

    ListItem::new(Line::from(vec![indicator, name, email]))
}

// ---- student picker (multi-select) ---------------------------

fn render_student_picker(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {
    let active = state.class_form.active_field == ClassFormField::StudentPicker;
    let n = state.class_form.selected_students.len();

    match &state.available_students {

        Resource::Idle | Resource::Loading => {
            let block = picker_block("Alunos".to_string(), active);
            frame.render_widget(
                Paragraph::new("Carregando...")
                    .style(Style::default().fg(Color::Yellow))
                    .block(block),
                area,
            );
        }

        Resource::Error(e) => {
            let block = picker_block("Alunos".to_string(), active);
            frame.render_widget(
                Paragraph::new(e.clone())
                    .style(Style::default().fg(theme::ERROR))
                    .block(block),
                area,
            );
        }

        Resource::Success(students) => {

            if students.is_empty() {
                let block = picker_block("Alunos".to_string(), active);
                frame.render_widget(
                    Paragraph::new("Nenhum aluno encontrado.")
                        .style(Style::default().fg(theme::KEY_LABEL))
                        .block(block),
                    area,
                );
                return;
            }

            let cursor   = state.class_form.student_cursor;
            let scroll   = state.class_form.student_scroll;
            let selected = &state.class_form.selected_students;
            let total    = students.len();

            let sel_label = format!(
                "{} selecionado{}",
                n,
                if n == 1 { "" } else { "s" },
            );
            let title = format!(
                "Alunos  ({}){}",
                sel_label,
                scroll_indicator(scroll, total, PICKER_VISIBLE_ROWS),
            );
            let block = picker_block(title, active);

            let items: Vec<ListItem> = students
                .iter()
                .enumerate()
                .skip(scroll)
                .take(PICKER_VISIBLE_ROWS)
                .map(|(i, user)| student_item(i, user, selected))
                .collect();

            let list = List::new(items)
                .highlight_style(
                    Style::default()
                        .bg(theme::LIST_SELECTED_BG)
                        .fg(theme::LIST_SELECTED_FG),
                )
                .block(block);

            let mut list_state = ListState::default();
            if active {
                list_state.select(Some(cursor.saturating_sub(scroll)));
            }

            frame.render_stateful_widget(list, area, &mut list_state);
        }
    }
}

fn student_item(
    idx: usize,
    user: &User,
    selected: &[usize],
) -> ListItem<'static> {

    let is_checked = selected.contains(&idx);

    let checkbox = if is_checked {
        Span::styled("[✓] ", Style::default().fg(Color::Green))
    } else {
        Span::styled("[ ] ", Style::default().fg(theme::KEY_LABEL))
    };

    let name = Span::styled(
        format!("{:<28}", user.name.clone()),
        if is_checked {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(theme::HEADER_FG)
        },
    );

    let email = Span::styled(
        user.email.clone(),
        Style::default().fg(theme::KEY_LABEL),
    );

    ListItem::new(Line::from(vec![checkbox, name, email]))
}

// ---- modal ConfirmDelete -------------------------------------

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
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(vertical[1]);

    let modal_area = horizontal[1];

    frame.render_widget(Clear, modal_area);

    let class_code = if let Resource::Success(ref list) = state.classes {
        list.get(state.selected_class_index)
            .map(|c| c.code.clone())
            .unwrap_or_default()
    } else {
        String::new()
    };

    let content = vec![
        Line::from(""),
        Line::from(
            Span::styled(
                format!("Remover turma \"{}\"?", class_code),
                Style::default().fg(theme::HEADER_FG),
            ),
        ),
        Line::from(
            Span::styled(
                "Esta ação também remove as tarefas da turma.",
                Style::default().fg(theme::KEY_LABEL),
            ),
        ),
        Line::from(""),
        Line::from(
            Span::styled(
                "Enter / y: confirmar   Esc / n: cancelar",
                Style::default().fg(theme::KEY_LABEL),
            ),
        ),
    ];

    frame.render_widget(
        Paragraph::new(content)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title("Confirmar exclusão")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme::ERROR)),
            ),
        modal_area,
    );
}
