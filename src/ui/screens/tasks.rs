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
            FIBONACCI_SCORES,
            PICKER_VISIBLE_ROWS,
            TaskFormField,
            TaskModal,
        },
    },
    models::class::ClassRoom,
    ui::theme,
};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let can_mutate = state.session
        .as_ref()
        .map(|s| s.role == "admin" || s.role == "teacher")
        .unwrap_or(false);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(5), Constraint::Length(7)])
        .split(area);

    render_list(frame, chunks[0], state, can_mutate);
    render_detail(frame, chunks[1], state);

    match state.task_modal {

        TaskModal::Add | TaskModal::Edit => {
            render_form_modal(frame, frame.area(), state);
        }

        TaskModal::ConfirmDelete => {
            render_confirm_modal(frame, frame.area(), state);
        }

        TaskModal::None => {}
    }
}

// ---- lista de tarefas ----------------------------------------

fn render_list(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    can_mutate: bool,
) {

    let content_focused = state.focus == Focus::Content;

    let border_style = if content_focused {
        Style::default().fg(theme::BORDER_FOCUSED)
    } else {
        Style::default().fg(theme::BORDER_INACTIVE)
    };

    match &state.tasks {

        Resource::Idle => {
            frame.render_widget(
                Paragraph::new("Nenhum dado carregado.")
                    .block(list_block("Tarefas", border_style)),
                area,
            );
        }

        Resource::Loading => {
            frame.render_widget(
                Paragraph::new("Carregando...")
                    .style(Style::default().fg(Color::Yellow))
                    .block(list_block("Tarefas", border_style)),
                area,
            );
        }

        Resource::Error(e) => {
            frame.render_widget(
                Paragraph::new(e.clone())
                    .style(Style::default().fg(theme::ERROR))
                    .block(list_block("Tarefas", border_style)),
                area,
            );
        }

        Resource::Success(tasks) => {

            if tasks.is_empty() {

                let msg = if can_mutate {
                    "Nenhuma tarefa cadastrada. Pressione 'a' para adicionar."
                } else {
                    "Nenhuma tarefa encontrada."
                };

                frame.render_widget(
                    Paragraph::new(msg)
                        .style(Style::default().fg(theme::KEY_LABEL))
                        .block(list_block("Tarefas (0)", border_style)),
                    area,
                );
                return;
            }

            let items: Vec<ListItem> = tasks
                .iter()
                .map(|t| {

                    let expires = t.expires_at
                        .as_deref()
                        .and_then(|s| s.get(..10))    // pega só a data YYYY-MM-DD
                        .unwrap_or("-");

                    ListItem::new(Line::from(vec![

                        Span::styled(
                            format!("{:<32}", t.title),
                            Style::default()
                                .fg(theme::HEADER_FG)
                                .add_modifier(Modifier::BOLD),
                        ),

                        Span::styled(
                            format!("{:<12}", t.class.code),
                            Style::default().fg(theme::BORDER_FOCUSED),
                        ),

                        Span::styled(
                            format!("{:>4} pts  ", t.score),
                            Style::default().fg(theme::ROLE_TEACHER),
                        ),

                        Span::styled(
                            format!("{:<24}", t.created_by.name),
                            Style::default().fg(theme::KEY_LABEL),
                        ),

                        Span::styled(
                            expires.to_string(),
                            Style::default().fg(theme::KEY_LABEL),
                        ),
                    ]))
                })
                .collect();

            let title = format!(
                "Tarefas ({})  [título · turma · pts · criado por · expira]",
                tasks.len(),
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
            list_state.select(Some(state.selected_task_index));

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

    let is_add = state.task_modal == TaskModal::Add;
    let picker_height = PICKER_VISIBLE_ROWS as u16 + 2;

    // Modal height:
    //   padding(1) + title(3) + desc(3) + score(3) + expires(3)
    //   + (picker em Add) + error(1) + help(1) + outer border(2)
    let min_height = if is_add {
        1 + 3 + 3 + 3 + 3 + picker_height + 1 + 1 + 2
    } else {
        1 + 3 + 3 + 3 + 3 + 1 + 1 + 2
    };

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

    let title = if is_add { "Nova Tarefa" } else { "Editar Tarefa" };

    let outer = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER_FOCUSED));

    frame.render_widget(outer, modal_area);

    // Layout interno
    let mut constraints = vec![
        Constraint::Length(1),  // [0] padding topo
        Constraint::Length(3),  // [1] Título
        Constraint::Length(3),  // [2] Descrição
        Constraint::Length(3),  // [3] Score
        Constraint::Length(3),  // [4] Expira em
    ];

    if is_add {
        constraints.push(Constraint::Length(picker_height)); // [5] ClassPicker
    }

    constraints.push(Constraint::Length(1)); // erro
    constraints.push(Constraint::Length(1)); // ajuda

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(modal_area);

    // Título
    render_text_field(
        frame,
        inner[1],
        "Título",
        &state.task_form.title,
        state.task_form.active_field == TaskFormField::Title,
    );

    // Descrição
    render_text_field(
        frame,
        inner[2],
        "Descrição  (opcional)",
        &state.task_form.description,
        state.task_form.active_field == TaskFormField::Description,
    );

    // Score — mostra o valor atual com indicação de que Espaço muda
    let score_val = FIBONACCI_SCORES[state.task_form.score_index];
    let score_active = state.task_form.active_field == TaskFormField::Score;

    let score_border = if score_active {
        Style::default().fg(theme::BORDER_FOCUSED)
    } else {
        Style::default().fg(theme::BORDER_INACTIVE)
    };

    let score_text = if score_active {
        format!("[ {} ]  ← Espaço para mudar", score_val)
    } else {
        format!("[ {} ]", score_val)
    };

    frame.render_widget(
        Paragraph::new(Span::styled(
            score_text,
            Style::default()
                .fg(theme::ROLE_TEACHER)
                .add_modifier(if score_active { Modifier::BOLD } else { Modifier::empty() }),
        ))
        .block(
            Block::default()
                .title("Score  (Fibonacci)")
                .borders(Borders::ALL)
                .border_style(score_border),
        ),
        inner[3],
    );

    // Expira em
    render_text_field(
        frame,
        inner[4],
        "Expira em  (ISO 8601, ex: 2026-06-30T23:59:59Z — opcional)",
        &state.task_form.expires_at,
        state.task_form.active_field == TaskFormField::ExpiresAt,
    );

    // Class picker (só no Add)
    let error_idx;
    let help_idx;

    if is_add {
        render_class_picker(frame, inner[5], state);
        error_idx = 6;
        help_idx  = 7;
    } else {
        error_idx = 5;
        help_idx  = 6;
    }

    // Erro inline
    if let Some(ref err) = state.error {
        frame.render_widget(
            Paragraph::new(
                Span::styled(err.as_str(), Style::default().fg(theme::ERROR)),
            )
            .alignment(Alignment::Center),
            inner[error_idx],
        );
    }

    // Ajuda
    frame.render_widget(
        Paragraph::new(
            "Tab: próximo campo   ↑/↓: navegar   Espaço: score/selecionar   Enter: salvar   Esc: cancelar",
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(theme::KEY_LABEL)),
        inner[help_idx],
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

// ---- class picker (single-select) ----------------------------

fn render_class_picker(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {
    let active = state.task_form.active_field == TaskFormField::ClassPicker;

    // Nome da turma selecionada para exibir no título
    let selected_code: Option<String> =
        if let (Some(idx), Resource::Success(classes)) =
            (state.task_form.selected_class, &state.available_task_classes)
        {
            classes.get(idx).map(|c| c.code.clone())
        } else {
            None
        };

    match &state.available_task_classes {

        Resource::Idle | Resource::Loading => {
            let block = picker_block("Turma".to_string(), active);
            frame.render_widget(
                Paragraph::new("Carregando...")
                    .style(Style::default().fg(Color::Yellow))
                    .block(block),
                area,
            );
        }

        Resource::Error(e) => {
            let block = picker_block("Turma".to_string(), active);
            frame.render_widget(
                Paragraph::new(e.clone())
                    .style(Style::default().fg(theme::ERROR))
                    .block(block),
                area,
            );
        }

        Resource::Success(classes) => {

            if classes.is_empty() {
                let block = picker_block("Turma".to_string(), active);
                frame.render_widget(
                    Paragraph::new("Nenhuma turma encontrada.")
                        .style(Style::default().fg(theme::KEY_LABEL))
                        .block(block),
                    area,
                );
                return;
            }

            let cursor   = state.task_form.class_cursor;
            let scroll   = state.task_form.class_scroll;
            let selected = state.task_form.selected_class;
            let total    = classes.len();

            let base = match selected_code {
                Some(ref code) => format!("Turma  ● {}", code),
                None           => "Turma  (nenhuma selecionada)".to_string(),
            };
            let title = format!("{}{}", base, scroll_indicator(scroll, total, PICKER_VISIBLE_ROWS));
            let block = picker_block(title, active);

            let items: Vec<ListItem> = classes
                .iter()
                .enumerate()
                .skip(scroll)
                .take(PICKER_VISIBLE_ROWS)
                .map(|(i, class)| class_picker_item(i, class, selected))
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

fn class_picker_item(
    idx: usize,
    class: &ClassRoom,
    selected: Option<usize>,
) -> ListItem<'static> {

    let is_selected = selected == Some(idx);

    let indicator = if is_selected {
        Span::styled("● ", Style::default().fg(Color::Green))
    } else {
        Span::styled("  ", Style::default())
    };

    let code = Span::styled(
        format!("{:<14}", class.code.clone()),
        if is_selected {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(theme::BORDER_FOCUSED)
        },
    );

    let label = Span::styled(
        format!("{} – {}", class.period.clone(), class.grade.clone()),
        Style::default().fg(theme::KEY_LABEL),
    );

    ListItem::new(Line::from(vec![indicator, code, label]))
}

// ---- detail panel --------------------------------------------

fn render_detail(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let block = Block::default()
        .title("Detalhes da Tarefa")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER_INACTIVE));

    match &state.tasks {

        Resource::Success(tasks) => {

            if let Some(t) = tasks.get(state.selected_task_index) {

                let expires = t.expires_at.as_deref().unwrap_or("—");
                let desc    = t.description.as_deref().unwrap_or("—");

                let lines = vec![
                    Line::from(vec![
                        Span::styled("Turma: ", Style::default().fg(theme::KEY_LABEL)),
                        Span::styled(t.class.code.clone(), Style::default().fg(theme::BORDER_FOCUSED)),
                        Span::styled("   Score: ", Style::default().fg(theme::KEY_LABEL)),
                        Span::styled(
                            format!("{} pts", t.score),
                            Style::default().fg(theme::ROLE_TEACHER),
                        ),
                        Span::styled("   Criado por: ", Style::default().fg(theme::KEY_LABEL)),
                        Span::styled(t.created_by.name.clone(), Style::default().fg(theme::HEADER_FG)),
                    ]),
                    Line::from(vec![
                        Span::styled("Expira: ", Style::default().fg(theme::KEY_LABEL)),
                        Span::raw(expires.to_string()),
                        Span::styled("   Criado em: ", Style::default().fg(theme::KEY_LABEL)),
                        Span::raw(t.created_at.get(..10).unwrap_or(&t.created_at).to_string()),
                    ]),
                    Line::from(vec![
                        Span::styled("Descrição: ", Style::default().fg(theme::KEY_LABEL)),
                        Span::raw(desc.to_string()),
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
                Paragraph::new("Selecione uma tarefa para ver os detalhes.")
                    .style(Style::default().fg(theme::KEY_LABEL))
                    .block(block),
                area,
            );
        }
    }
}

// ---- modal de confirmação de exclusão ------------------------

fn render_confirm_modal(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
) {

    let title_text = if let Resource::Success(ref list) = state.tasks {
        list.get(state.selected_task_index)
            .map(|t| t.title.clone())
            .unwrap_or_default()
    } else {
        String::new()
    };

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(7),
            Constraint::Percentage(40),
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

    let inner = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(modal_area);

    let outer = Block::default()
        .title("Confirmar exclusão")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::ERROR));

    frame.render_widget(outer, modal_area);

    frame.render_widget(
        Paragraph::new(format!("Remover: \"{}\"?", title_text))
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme::HEADER_FG)),
        inner[1],
    );

    frame.render_widget(
        Paragraph::new("Esta ação não pode ser desfeita.")
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme::KEY_LABEL)),
        inner[2],
    );

    frame.render_widget(
        Paragraph::new("Enter / y: confirmar   Esc / n: cancelar")
            .alignment(Alignment::Center)
            .style(Style::default().fg(theme::KEY_HINT)),
        inner[3],
    );
}
