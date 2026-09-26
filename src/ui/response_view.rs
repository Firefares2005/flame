use crate::app::App;
use crate::config;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let Some(res) = &app.response else {
        let p = Paragraph::new("No response yet. Send a request first (F5).")
            .block(Block::default().borders(Borders::ALL).title(" Response "))
            .style(Style::default().fg(config::MUTED_COLOR));
        f.render_widget(p, area);
        return;
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Min(5),
        ])
        .split(area);

    let status_color = match res.status {
        200..=299 => config::STATUS_SUCCESS,
        300..=399 => config::STATUS_REDIRECT,
        400..=499 => config::STATUS_CLIENT_ERR,
        500..=599 => config::STATUS_SERVER_ERR,
        _ => Color::Gray,
    };

    let status_line = Line::from(vec![
        Span::styled(
            format!(" {} ", res.status),
            Style::default()
                .fg(Color::Black)
                .bg(status_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(&res.status_text, Style::default().fg(status_color)),
        Span::raw("  "),
        Span::styled(
            format!("⏱ {} ms", res.duration_ms),
            Style::default().fg(config::ACCENT_COLOR),
        ),
        Span::raw("  "),
        Span::styled(
            format!("[scroll: {}]", app.response_scroll),
            Style::default().fg(config::MUTED_COLOR),
        ),
    ]);

    let status_p = Paragraph::new(status_line).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Status ")
            .border_style(Style::default().fg(config::BORDER_COLOR)),
    );
    f.render_widget(status_p, chunks[0]);

    let header_lines: Vec<Line> = res
        .headers
        .iter()
        .map(|(k, v)| {
            Line::from(vec![
                Span::styled(format!("{}: ", k), Style::default().fg(config::ACCENT_COLOR)),
                Span::raw(v.as_str()),
            ])
        })
        .collect();

    let headers_p = Paragraph::new(header_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Headers ")
                .border_style(Style::default().fg(config::BORDER_COLOR)),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(headers_p, chunks[1]);

    let body_lines = colorize_body(&res.pretty_body);
    let body_p = Paragraph::new(body_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Body  (↑↓ / PgUp PgDn to scroll) ")
                .border_style(Style::default().fg(config::BORDER_COLOR)),
        )
        .wrap(Wrap { trim: false })
        .scroll((app.response_scroll, 0));
    f.render_widget(body_p, chunks[2]);
}

fn colorize_body(text: &str) -> Vec<Line<'_>> {
    text.lines()
        .map(|line| {
            let trimmed = line.trim_start();
            let style = if trimmed.starts_with('"') && trimmed.contains(':') {
                Style::default().fg(Color::Green)
            } else if trimmed.starts_with('"') {
                Style::default().fg(Color::Yellow)
            } else if trimmed.starts_with('{') || trimmed.starts_with('}') {
                Style::default().fg(config::ACCENT_COLOR)
            } else if trimmed.starts_with('[') || trimmed.starts_with(']') {
                Style::default().fg(config::ACCENT_COLOR)
            } else {
                Style::default().fg(Color::White)
            };
            Line::from(Span::styled(line.to_string(), style))
        })
        .collect()
}