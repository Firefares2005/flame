use crate::app::App;
use crate::config;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_full(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_collections(f, app, chunks[0]);
    render_history(f, app, chunks[1]);
}

fn render_collections(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .collections
        .iter()
        .enumerate()
        .map(|(i, req)| {
            let style = if i == app.sidebar_selected {
                Style::default()
                    .fg(config::ACCENT_COLOR)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("[{}] ", req.method), Style::default().fg(ratatui::style::Color::Cyan)),
                Span::styled(req.url.clone(), style),
            ]))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" Collections ({}) ", app.collections.len()))
            .border_style(Style::default().fg(config::BORDER_COLOR)),
    );

    f.render_widget(list, area);
}

fn render_history(f: &mut Frame, app: &App, area: Rect) {
    if app.history.is_empty() {
        let p = Paragraph::new("No history yet.")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" History ")
                    .border_style(Style::default().fg(config::BORDER_COLOR)),
            )
            .style(Style::default().fg(config::MUTED_COLOR));
        f.render_widget(p, area);
        return;
    }

    let items: Vec<ListItem> = app
        .history
        .iter()
        .map(|h| {
            let color = match h.status {
                200..=299 => config::STATUS_SUCCESS,
                300..=399 => config::STATUS_REDIRECT,
                400..=499 => config::STATUS_CLIENT_ERR,
                500..=599 => config::STATUS_SERVER_ERR,
                _ => ratatui::style::Color::Gray,
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", h.method), Style::default().fg(ratatui::style::Color::Cyan)),
                Span::styled(format!("[{}] ", h.status), Style::default().fg(color)),
                Span::raw(h.url.clone()),
                Span::styled(
                    format!("  ({}ms)", h.duration_ms),
                    Style::default().fg(config::MUTED_COLOR),
                ),
            ]))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!(" History ({}) ", app.history.len()))
            .border_style(Style::default().fg(config::BORDER_COLOR)),
    );

    f.render_widget(list, area);
}