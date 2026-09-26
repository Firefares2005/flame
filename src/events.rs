use crate::app::{App, Tab};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub async fn handle_key(app: &mut App, key: KeyEvent) -> Result<()> {
    // ============ ALT + KEY ============
    if key.modifiers.contains(KeyModifiers::ALT) {
        match key.code {
            KeyCode::Char('m') => {
                app.method = app.method.next();
                return Ok(());
            }
            KeyCode::Enter => {
                app.send_request();
                return Ok(());
            }
            _ => {}
        }
    }

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