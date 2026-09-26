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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // method + url
            Constraint::Length(5), // headers
            Constraint::Min(5),    // body
        ])
        .split(area);

    render_url_bar(f, app, chunks[0]);
    render_headers(f, app, chunks[1]);
    render_body(f, app, chunks[2]);
}

fn render_url_bar(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(12), Constraint::Min(20)])
        .split(area);

    let method_color = match app.method.as_str() {
        "GET" => Color::Green,
        "POST" => Color::Yellow,
        "PUT" => Color::Blue,
        "DELETE" => Color::Red,
        "PATCH" => Color::Magenta,
        _ => Color::Gray,
    };

    let method_block = Block::default()
        .borders(Borders::ALL)
        .border_style(if app.focus == 0 {
            Style::default().fg(config::ACTIVE_BORDER)
        } else {
            Style::default().fg(config::BORDER_COLOR)
        });
    let method_p = Paragraph::new(Line::from(Span::styled(
        format!(" {} ", app.method.as_str()),
        Style::default()
            .fg(method_color)
            .add_modifier(Modifier::BOLD),
    )))
    .block(method_block);
    f.render_widget(method_p, chunks[0]);

    let url_block = Block::default()
        .borders(Borders::ALL)
        .title(" URL ")
        .border_style(if app.focus == 0 {
            Style::default().fg(config::ACTIVE_BORDER)
        } else {
            Style::default().fg(config::BORDER_COLOR)
        });
    let url_p = Paragraph::new(app.url.as_str()).block(url_block);
    f.render_widget(url_p, chunks[1]);

    // Place cursor if focused
    if app.focus == 0 {
        let x = chunks[1].x + 1 + app.url.len() as u16;
        let y = chunks[1].y + 1;
        if x < chunks[1].x + chunks[1].width - 1 {
            f.set_cursor_position((x, y));
        }
    }
}

fn render_headers(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Headers (key: value per line) ")
        .border_style(if app.focus == 1 {
            Style::default().fg(config::ACTIVE_BORDER)
        } else {
            Style::default().fg(config::BORDER_COLOR)
        });

    let p = Paragraph::new(app.headers_input.as_str())
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(p, area);

    if app.focus == 1 {
        let last_line_len = app
            .headers_input
            .lines()
            .last()
            .map(|l| l.len())
            .unwrap_or(0) as u16;
        let lines = app.headers_input.lines().count().max(1) as u16;
        let x = area.x + 1 + last_line_len;
        let y = area.y + lines;
        if x < area.x + area.width - 1 && y < area.y + area.height - 1 {
            f.set_cursor_position((x, y));
        }
    }
}

fn render_body(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Body ")
        .border_style(if app.focus == 2 {
            Style::default().fg(config::ACTIVE_BORDER)
        } else {
            Style::default().fg(config::BORDER_COLOR)
        });

    let p = Paragraph::new(app.body_input.as_str())
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(p, area);

    if app.focus == 2 {
        let last_line_len = app
            .body_input
            .lines()
            .last()
            .map(|l| l.len())
            .unwrap_or(0) as u16;
        let lines = app.body_input.lines().count().max(1) as u16;
        let x = area.x + 1 + last_line_len;
        let y = area.y + lines;
        if x < area.x + area.width - 1 && y < area.y + area.height - 1 {
            f.set_cursor_position((x, y));
        }
    }
}