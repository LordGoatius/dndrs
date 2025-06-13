#![feature(iter_intersperse)]
pub mod config;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

use anyhow;
use clap::Parser;
use config::Config;
use dnd::character::Character;
use event::{Event, EventHandler};
use handler::*;
use ratatui::{backend::CrosstermBackend, widgets::{RatatuiLogoSize, Widget}, Terminal};
use std::fs;
use tui::Tui;

pub struct App<'a> {
    pub(crate) character: Character<'a>,
    pub(crate) should_quit: bool,
}

fn main() -> anyhow::Result<()> {
    let config = Config::parse();
    let str =
        &fs::read_to_string(config.path).expect("Please use a valid path to a character file");
    let character = ron::from_str::<Character>(str).expect("Invalid character file.");

    let mut app = App {
        character: character,
        should_quit: false,
    };

    let terminal = ratatui::init();
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);

    while !app.should_quit {
        tui.draw(&mut app)?;
        match tui.events.next()? {
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(mouse_event) => handle_mouse_events(mouse_event, &mut app)?,
            _ => (),
        };
    }

    ratatui::restore();

    Ok(())
}
