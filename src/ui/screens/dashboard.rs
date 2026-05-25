use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::{
    app::{
        resources::Resource,
        state::AppState,
    },
    models::dashboard::{AdminDashboard, DashboardData, StudentDashboard, TeacherDashboard},
    ui::theme,
};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {

    match &state.dashboard {

        Resource::Idle | Resource::Loading => render_loading(frame, area),

        Resource::Error(msg) => render_error(frame, area, msg.clone()),

        Resource::Success(data) => match data {
            DashboardData::Admin(d)   => render_admin(frame, area, d),
            DashboardData::Teacher(d) => render_teacher(frame, area, d),
            DashboardData::Student(d) => render_student(frame, area, d),
        }
    }
}

// ---- estados --------------------------------------------------

fn render_loading(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Carregando...")
            .style(Style::default().fg(theme::ROLE_TEACHER))
            .block(titled_block("Dashboard"))
            .alignment(Alignment::Center),
        area,
    );
}

fn render_error(frame: &mut Frame, area: Rect, msg: String) {
    frame.render_widget(
        Paragraph::new(msg)
            .style(Style::default().fg(theme::ERROR))
            .block(titled_block("Dashboard"))
            .wrap(Wrap { trim: false }),
        area,
    );
}

// ---- Admin ----------------------------------------------------

fn render_admin(frame: &mut Frame, area: Rect, d: &AdminDashboard) {

    let outer = titled_block("Dashboard");
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(inner);

    frame.render_widget(
        card_lines(
            "Usuários",
            vec![
                stat_line("Total:      ", d.users.total),
                stat_line("Admins:     ", d.users.admins),
                stat_line("Professores:", d.users.teachers),
                stat_line("Alunos:     ", d.users.students),
            ],
            theme::ROLE_ADMIN,
        ),
        cols[0],
    );

    frame.render_widget(
        card_lines(
            "Turmas",
            vec![stat_line("Total:", d.classes.total)],
            Color::Cyan,
        ),
        cols[1],
    );

    frame.render_widget(
        card_lines(
            "Tarefas",
            vec![
                stat_line("Total:      ", d.tasks.total),
                stat_line("Submissões: ", d.tasks.submissions),
                stat_line("Pendentes:  ", d.tasks.pending_grades),
            ],
            theme::ROLE_TEACHER,
        ),
        cols[2],
    );
}

// ---- Teacher --------------------------------------------------

fn render_teacher(frame: &mut Frame, area: Rect, d: &TeacherDashboard) {

    let outer = titled_block("Dashboard");
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(1)])
        .split(inner);

    // Overview
    let overview = Line::from(vec![
        Span::raw(format!("Turmas: {}   ", d.overview.total_classes)),
        Span::raw(format!("Alunos: {}   ", d.overview.total_students)),
        Span::raw(format!("Tarefas: {}   ", d.overview.total_tasks)),
        Span::styled(
            format!("Pendentes: {}", d.overview.pending_grades),
            Style::default().fg(theme::ROLE_TEACHER),
        ),
    ]);

    frame.render_widget(
        Paragraph::new(vec![overview])
            .block(section_block("Visão Geral")),
        rows[0],
    );

    // Turmas
    let mut lines: Vec<Line<'static>> = d.classes.iter().map(|c| {
        Line::from(vec![
            Span::styled(
                format!("{:<12}", c.code),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "   Alunos: {}   Tarefas: {}   ",
                c.student_count, c.task_count
            )),
            Span::styled(
                format!("Pendentes: {}", c.pending_grades),
                Style::default().fg(theme::ROLE_TEACHER),
            ),
        ])
    }).collect();

    if lines.is_empty() {
        lines.push(Line::from(Span::styled(
            "Nenhuma turma encontrada.",
            Style::default().fg(theme::BORDER_INACTIVE),
        )));
    }

    frame.render_widget(
        Paragraph::new(lines)
            .block(section_block("Minhas Turmas"))
            .wrap(Wrap { trim: false }),
        rows[1],
    );
}

// ---- Student --------------------------------------------------

fn render_student(frame: &mut Frame, area: Rect, d: &StudentDashboard) {

    let outer = titled_block("Dashboard");
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(5),
            Constraint::Min(0),
        ])
        .split(inner);

    // Tarefas
    let tasks_lines = vec![
        Line::from(vec![
            Span::raw(format!("Total: {}   ", d.tasks.total)),
            Span::raw(format!("Enviadas: {}   ", d.tasks.submitted)),
            Span::styled(
                format!("Pendentes: {}", d.tasks.pending),
                Style::default().fg(theme::ROLE_TEACHER),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                format!("Expiradas: {}   ", d.tasks.expired),
                Style::default().fg(theme::ERROR),
            ),
            Span::raw(format!("Corrigidas: {}", d.tasks.graded)),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(tasks_lines).block(section_block("Tarefas")),
        rows[0],
    );

    // Pontuação
    let score_line = Line::from(vec![
        Span::styled(
            format!("Ganho: {} pts   ", d.score.total_earned),
            Style::default().fg(theme::SUCCESS),
        ),
        Span::raw(format!("Possível: {} pts   ", d.score.total_possible)),
        Span::styled(
            format!("Média: {:.1} pts", d.score.average),
            Style::default().fg(Color::Cyan),
        ),
    ]);

    frame.render_widget(
        Paragraph::new(vec![score_line]).block(section_block("Pontuação")),
        rows[1],
    );
}

// ---- helpers --------------------------------------------------

fn titled_block(title: &str) -> Block<'_> {
    Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER_INACTIVE))
}

fn section_block(title: &str) -> Block<'static> {
    Block::default()
        .title(format!(" {} ", title))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER_INACTIVE))
}

fn card_lines(title: &str, lines: Vec<Line<'static>>, accent: Color) -> Paragraph<'static> {
    Paragraph::new(lines).block(
        Block::default()
            .title(format!(" {} ", title))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(accent)),
    )
}

fn stat_line(label: &str, value: u32) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            format!("  {label} "),
            Style::default().fg(Color::Gray),
        ),
        Span::styled(
            value.to_string(),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
    ])
}
