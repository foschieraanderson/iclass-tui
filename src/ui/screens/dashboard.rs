use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::{
    app::{resources::Resource, state::AppState},
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
        },
    }
}

// ---- loading / error ------------------------------------------

fn render_loading(frame: &mut Frame, area: Rect) {
    frame.render_widget(
        Paragraph::new("Carregando...")
            .style(Style::default().fg(theme::ROLE_TEACHER))
            .block(outer_block("Dashboard"))
            .alignment(Alignment::Center),
        area,
    );
}

fn render_error(frame: &mut Frame, area: Rect, msg: String) {
    frame.render_widget(
        Paragraph::new(msg)
            .style(Style::default().fg(theme::ERROR))
            .block(outer_block("Dashboard"))
            .wrap(Wrap { trim: false }),
        area,
    );
}

// ---- Admin: 2 linhas ------------------------------------------
//
// Linha 1 (altura 8): cards stat Usuários | Turmas | Tarefas
// Linha 2 (restante): BarChart distribuição de usuários | Gauge correções pendentes

fn render_admin(frame: &mut Frame, area: Rect, d: &AdminDashboard) {
    let outer = outer_block("Dashboard");
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(1)])
        .split(inner);

    // -- Linha 1: cards stat --
    let cards = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(rows[0]);

    frame.render_widget(
        stat_card(
            "Usuários",
            vec![
                stat_line("Total:      ", d.users.total),
                stat_line("Admins:     ", d.users.admins),
                stat_line("Professores:", d.users.teachers),
                stat_line("Alunos:     ", d.users.students),
            ],
            theme::ROLE_ADMIN,
        ),
        cards[0],
    );

    frame.render_widget(
        stat_card(
            "Turmas",
            vec![stat_line("Total:", d.classes.total)],
            Color::Cyan,
        ),
        cards[1],
    );

    frame.render_widget(
        stat_card(
            "Tarefas",
            vec![
                stat_line("Total:      ", d.tasks.total),
                stat_line("Submissões: ", d.tasks.submissions),
                stat_line("Pendentes:  ", d.tasks.pending_grades),
            ],
            theme::ROLE_TEACHER,
        ),
        cards[2],
    );

    // -- Linha 2: BarChart + Gauge --
    let bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(26)])
        .split(rows[1]);

    frame.render_widget(
        BarChart::default()
            .block(section_block("Distribuição de Usuários"))
            .bar_width(9)
            .bar_gap(1)
            .bar_style(Style::default().fg(Color::Cyan))
            .value_style(Style::default().fg(Color::Black).bg(Color::Cyan))
            .label_style(Style::default().fg(Color::Gray))
            .data(
                BarGroup::default().bars(&[
                    Bar::default().label(Line::from("Admin")).value(d.users.admins as u64),
                    Bar::default().label(Line::from("Prof.")).value(d.users.teachers as u64),
                    Bar::default().label(Line::from("Alunos")).value(d.users.students as u64),
                ])
            ),
        bottom[0],
    );

    let ratio = if d.tasks.submissions > 0 {
        (d.tasks.pending_grades as f64 / d.tasks.submissions as f64).min(1.0)
    } else {
        0.0
    };
    frame.render_widget(
        Gauge::default()
            .block(section_block("Correções Pendentes"))
            .gauge_style(Style::default().fg(Color::Yellow).bg(Color::DarkGray))
            .ratio(ratio)
            .label(format!(
                "{}/{} ({:.0}%)",
                d.tasks.pending_grades,
                d.tasks.submissions,
                ratio * 100.0,
            )),
        bottom[1],
    );
}

// ---- Teacher: visão geral + 2 BarCharts -----------------------
//
// Linha 1 (altura 4): card overview
// Linha 2 (restante): BarChart alunos por turma | BarChart correções por turma

fn render_teacher(frame: &mut Frame, area: Rect, d: &TeacherDashboard) {
    let outer = outer_block("Dashboard");
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(4), Constraint::Min(1)])
        .split(inner);

    // -- Linha 1: overview --
    let overview = Line::from(vec![
        Span::raw(format!("Turmas: {}   ", d.overview.total_classes)),
        Span::raw(format!("Alunos: {}   ", d.overview.total_students)),
        Span::raw(format!("Tarefas: {}   ", d.overview.total_tasks)),
        Span::styled(
            format!("Correções pendentes: {}", d.overview.pending_grades),
            Style::default().fg(theme::ROLE_TEACHER),
        ),
    ]);
    frame.render_widget(
        Paragraph::new(vec![overview]).block(section_block("Visão Geral")),
        rows[0],
    );

    // -- Linha 2: 2 BarCharts --
    let charts = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[1]);

    // BarChart alunos por turma
    let student_bars: Vec<Bar<'static>> = d.classes.iter().map(|c| {
        Bar::default()
            .label(Line::from(grade_label(&c.code)))
            .value(c.student_count as u64)
    }).collect();

    if student_bars.is_empty() {
        frame.render_widget(
            Paragraph::new("Nenhuma turma.")
                .style(Style::default().fg(theme::BORDER_INACTIVE))
                .block(section_block("Alunos por Turma"))
                .alignment(Alignment::Center),
            charts[0],
        );
    } else {
        frame.render_widget(
            BarChart::default()
                .block(section_block("Alunos por Turma"))
                .bar_width(6)
                .bar_gap(1)
                .bar_style(Style::default().fg(Color::Cyan))
                .value_style(Style::default().fg(Color::Black).bg(Color::Cyan))
                .label_style(Style::default().fg(Color::Gray))
                .data(BarGroup::default().bars(&student_bars)),
            charts[0],
        );
    }

    // BarChart correções pendentes por turma
    let pending_bars: Vec<Bar<'static>> = d.classes.iter().map(|c| {
        Bar::default()
            .label(Line::from(grade_label(&c.code)))
            .value(c.pending_grades as u64)
            .style(Style::default().fg(Color::Yellow))
    }).collect();

    if pending_bars.is_empty() {
        frame.render_widget(
            Paragraph::new("Nenhuma turma.")
                .style(Style::default().fg(theme::BORDER_INACTIVE))
                .block(section_block("Correções Pendentes por Turma"))
                .alignment(Alignment::Center),
            charts[1],
        );
    } else {
        frame.render_widget(
            BarChart::default()
                .block(section_block("Correções Pendentes por Turma"))
                .bar_width(6)
                .bar_gap(1)
                .bar_style(Style::default().fg(Color::Yellow))
                .value_style(Style::default().fg(Color::Black).bg(Color::Yellow))
                .label_style(Style::default().fg(Color::Gray))
                .data(BarGroup::default().bars(&pending_bars)),
            charts[1],
        );
    }
}

// ---- Student: 2 Gauges (esq) + BarChart status (dir) ----------
//
// Coluna esquerda (40%): Gauge tarefas entregues | Gauge pontuação
// Coluna direita:        BarChart 4 barras por status

fn render_student(frame: &mut Frame, area: Rect, d: &StudentDashboard) {
    let outer = outer_block("Dashboard");
    let inner = outer.inner(area);
    frame.render_widget(outer, area);

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Min(1)])
        .split(inner);

    // -- Coluna esquerda: 2 Gauges --
    let gauges = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(cols[0]);

    let task_ratio = if d.tasks.total > 0 {
        (d.tasks.submitted as f64 / d.tasks.total as f64).min(1.0)
    } else {
        0.0
    };
    frame.render_widget(
        Gauge::default()
            .block(section_block("Tarefas Entregues"))
            .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
            .ratio(task_ratio)
            .label(format!(
                "{} de {}  ({:.0}%)",
                d.tasks.submitted,
                d.tasks.total,
                task_ratio * 100.0,
            )),
        gauges[0],
    );

    let score_ratio = if d.score.total_possible > 0 {
        (d.score.total_earned as f64 / d.score.total_possible as f64).min(1.0)
    } else {
        0.0
    };
    frame.render_widget(
        Gauge::default()
            .block(section_block("Pontuação"))
            .gauge_style(Style::default().fg(theme::SUCCESS).bg(Color::DarkGray))
            .ratio(score_ratio)
            .label(format!(
                "{} de {} pts  ({:.0}%)",
                d.score.total_earned,
                d.score.total_possible,
                score_ratio * 100.0,
            )),
        gauges[1],
    );

    // -- Coluna direita: BarChart status --
    frame.render_widget(
        BarChart::default()
            .block(section_block("Status das Tarefas"))
            .bar_width(9)
            .bar_gap(1)
            .label_style(Style::default().fg(Color::Gray))
            .data(
                BarGroup::default().bars(&[
                    Bar::default()
                        .label(Line::from("Enviadas"))
                        .value(d.tasks.submitted as u64)
                        .style(Style::default().fg(Color::Cyan))
                        .value_style(Style::default().fg(Color::Black).bg(Color::Cyan)),
                    Bar::default()
                        .label(Line::from("Pendentes"))
                        .value(d.tasks.pending as u64)
                        .style(Style::default().fg(Color::Yellow))
                        .value_style(Style::default().fg(Color::Black).bg(Color::Yellow)),
                    Bar::default()
                        .label(Line::from("Expiradas"))
                        .value(d.tasks.expired as u64)
                        .style(Style::default().fg(Color::Red))
                        .value_style(Style::default().fg(Color::Black).bg(Color::Red)),
                    Bar::default()
                        .label(Line::from("Corrigidas"))
                        .value(d.tasks.graded as u64)
                        .style(Style::default().fg(theme::SUCCESS))
                        .value_style(Style::default().fg(Color::Black).bg(theme::SUCCESS)),
                ])
            ),
        cols[1],
    );
}

// ---- helpers --------------------------------------------------

fn outer_block(title: &str) -> Block<'_> {
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

fn stat_card(title: &str, lines: Vec<Line<'static>>, accent: Color) -> Paragraph<'static> {
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

/// Extrai a série/turma do código para usar como label curto no BarChart.
/// "2026/1-3A" → "3A"
fn grade_label(code: &str) -> String {
    code.rsplit('-').next().unwrap_or(code).to_string()
}
