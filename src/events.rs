use crate::app::{App, Tab};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub async fn handle_key(app: &mut App, key: KeyEvent) -> Result<()> {
    app.status_message = None;

    // ============ CTRL + KEY ============
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('r') => {
                app.active_tab = Tab::Request;
                return Ok(());
            }
            KeyCode::Char('e') => {
                app.active_tab = Tab::Response;
                return Ok(());
            }
            KeyCode::Char('l') => {
                app.active_tab = Tab::Collections;
                return Ok(());
            }
            KeyCode::Char('s') => {
                app.save_current();
                return Ok(());
            }
            KeyCode::Char('n') => {
                app.focus_next();
                return Ok(());
            }
            KeyCode::Char('p') => {
                app.focus_prev();
                return Ok(());
            }
            KeyCode::Char('m') => {
                app.method = app.method.next();
                return Ok(());
            }
            KeyCode::Char('j') => {
                app.method = app.method.prev();
                return Ok(());
            }
            KeyCode::Enter => {
                app.send_request();
                return Ok(());
            }
            _ => {}
        }
    }

    // ============ F-KEYS (بدائل) ============
    match key.code {
        KeyCode::F(2) => {
            app.method = app.method.next();
            return Ok(());
        }
        KeyCode::F(5) => {
            app.send_request();
            return Ok(());
        }
        _ => {}
    }

    // ============ SCROLLING (Response tab) ============
    if app.active_tab == Tab::Response {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                app.response_scroll = app.response_scroll.saturating_sub(1);
                return Ok(());
            }
            KeyCode::Down | KeyCode::Char('j') => {
                app.response_scroll = app.response_scroll.saturating_add(1);
                return Ok(());
            }
            KeyCode::PageUp => {
                app.response_scroll = app.response_scroll.saturating_sub(10);
                return Ok(());
            }
            KeyCode::PageDown => {
                app.response_scroll = app.response_scroll.saturating_add(10);
                return Ok(());
            }
            KeyCode::Home | KeyCode::Char('g') => {
                app.response_scroll = 0;
                return Ok(());
            }
            _ => {}
        }
    }

    // ============ TAB NAVIGATION ============
    match app.active_tab {
        Tab::Collections => match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if app.sidebar_selected > 0 {
                    app.sidebar_selected -= 1;
                }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if app.sidebar_selected + 1 < app.collections.len() {
                    app.sidebar_selected += 1;
                }
            }
            KeyCode::Enter => {
                app.load_selected_collection();
            }
            KeyCode::Char('d') | KeyCode::Delete => {
                app.delete_selected_collection();
            }
            _ => {}
        },
        _ => match key.code {
            KeyCode::Tab => app.focus_next(),
            KeyCode::BackTab => app.focus_prev(),
            KeyCode::Char(c) => {
                app.current_input_mut().push(c);
            }
            KeyCode::Backspace => {
                app.current_input_mut().pop();
            }
            KeyCode::Esc => {
                app.active_tab = Tab::Request;
            }
            _ => {}
        },
    }

    Ok(())
}