mod request_view;
mod response_view;
mod sidebar;

use crate::app::{App, Tab};
use crate::config;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Tabs},
    Frame,
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // title + tabs
            Constraint::Min(10),   // main
            Constraint::Length(2), // status bar
        ])
        .split(f.area());

    render_header(f, app, chunks[0]);
    render_main(f, app, chunks[1]);
    render_status(f, app, chunks[2]);
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = vec![
        Line::from(Span::styled(" 1 Request ", Style::default().fg(Color::White))),
        Line::from(Span::styled(" 2 Response ", Style::default().fg(Color::White))),
        Line::from(Span::styled(" 3 Collections ", Style::default().fg(Color::White))),
    ];

    let selected = match app.active_tab {
        Tab::Request => 0,
        Tab::Response => 1,
        Tab::Collections => 2,
    };

    let tabs = Tabs::new(titles)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(Span::styled(
                    " 🔥 flame ",
                    Style::default()
                        .fg(config::TITLE_COLOR)
                        .add_modifier(Modifier::BOLD),
                ))
                .border_style(Style::default().fg(config::BORDER_COLOR)),
        )
        .select(selected)
        .highlight_style(
            Style::default()
                .fg(config::ACCENT_COLOR)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(tabs, area);
}

fn render_main(f: &mut Frame, app: &App, area: Rect) {
    match app.active_tab {
        Tab::Request => request_view::render(f, app, area),
        Tab::Response => response_view::render(f, app, area),
        Tab::Collections => sidebar::render_full(f, app, area),
    }
}

fn render_status(f: &mut Frame, app: &App, area: Rect) {
    let (msg, color) = if app.loading {
        ("⏳ Sending request...".to_string(), config::ACCENT_COLOR)
    } else if let Some(err) = &app.error {
        (format!("❌ {}", err), Color::Red)
    } else if let Some(res) = &app.response {
        (
            format!(
                "✅ {} {} — {} ms",
                res.status, res.status_text, res.duration_ms
            ),
            Color::Green,
        )
    } else {
        (
            "Tab: switch field │ Alt+Enter / Ctrl+Enter: send │ Ctrl+S: save │ Ctrl+C: quit"
                .to_string(),
            config::MUTED_COLOR,
        )
    };

    let p = Paragraph::new(Line::from(Span::styled(msg, Style::default().fg(color))))
        .style(Style::default().bg(Color::Black));
    f.render_widget(p, area);
}