use ratatui::{
    prelude::*,
    widgets::*,
};

use crate::{
    app::{resources::Resource, state::AppState},
    models::report::{ClassReport, ReportStudent},
    ui::theme,
};

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    match &state.reports {
        Resource::Idle | Resource::Loading => {
            frame.render_widget(
                Paragraph::new("Carregando...")
                    .style(Style::default().fg(theme::ROLE_TEACHER))
                    .block(outer_block())
                    .alignment(Alignment::Center),
                area,
            );
        }

        Resource::Error(msg) => {
            frame.render_widget(
                Paragraph::new(msg.clone())
                    .style(Style::default().fg(theme::ERROR))
                    .block(outer_block())
                    .wrap(Wrap { trim: false }),
                area,
            );
        }

        Resource::Success(reports) if reports.is_empty() => {
            frame.render_widget(
                Paragraph::new("Nenhuma turma encontrada.")
                    .style(Style::default().fg(theme::BORDER_INACTIVE))
                    .block(outer_block())
                    .alignment(Alignment::Center),
                area,
            );
        }

        Resource::Success(reports) => {
            let n = reports.len() as u32;
            let constraints: Vec<Constraint> = reports.iter()
                .map(|_| Constraint::Ratio(1, n))
                .collect();

            let rows = Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraints)
                .split(area);

            for (report, &row_area) in reports.iter().zip(rows.iter()) {
                render_class_chart(frame, row_area, report);
            }
        }
    }
}

fn render_class_chart(frame: &mut Frame, area: Rect, report: &ClassReport) {
    let total_possible = report.tasks.iter().map(|t| t.score).sum::<u32>();
    let total_tasks = report.tasks.len();

    let active_students: Vec<&ReportStudent> = report.students.iter()
        .filter(|s| s.submissions.iter().any(|sub| sub.submitted))
        .collect();

    let bars: Vec<Bar<'static>> = active_students.iter().map(|s| {
        let submitted_count = s.submissions.iter().filter(|sub| sub.submitted).count();
        let label = format!("{} ({}/{})", short_name(&s.name), submitted_count, total_tasks);
        Bar::default()
            .label(Line::from(label))
            .value(s.total_earned as u64)
    }).collect();

    let title = format!(
        " {} — {} pts possíveis ",
        report.class_code,
        total_possible,
    );

    if bars.is_empty() {
        frame.render_widget(
            Paragraph::new("Nenhuma entrega ainda.")
                .style(Style::default().fg(theme::BORDER_INACTIVE))
                .block(
                    Block::default()
                        .title(title)
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(theme::BORDER_INACTIVE)),
                )
                .alignment(Alignment::Center),
            area,
        );
        return;
    }

    frame.render_widget(
        BarChart::default()
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(theme::BORDER_INACTIVE)),
            )
            .bar_width(10)
            .bar_gap(1)
            .bar_style(Style::default().fg(Color::Cyan))
            .value_style(Style::default().fg(Color::Black).bg(Color::Cyan))
            .label_style(Style::default().fg(Color::Gray))
            .data(BarGroup::default().bars(&bars)),
        area,
    );
}

fn outer_block() -> Block<'static> {
    Block::default()
        .title(" Relatórios ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme::BORDER_INACTIVE))
}

fn short_name(name: &str) -> String {
    name.split_whitespace().next().unwrap_or(name).to_string()
}
