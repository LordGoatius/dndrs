use std::str::FromStr;

use crate::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent, MouseEventKind, MouseButton};
use crossterm::terminal::size;
use anyhow::{Result, Ok};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> Result<()> {
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            app.should_quit = true;
        }
        // Exit application on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.should_quit = true;
            }
        }
        // Counter handlers
        // KeyCode::Right => {
        //     app.increment_counter();
        // }
        // KeyCode::Left => {
        //     app.decrement_counter();
        // }
        // KeyCode::Char('j') => {
        //     app.decrement_counter();
        // },
        // KeyCode::Char('k') => {
        //     app.increment_counter();
        // }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}

pub fn handle_mouse_events(mouse_event: MouseEvent, app: &mut App) -> Result<()> {
    Ok(())
}
