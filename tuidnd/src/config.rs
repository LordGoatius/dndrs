use std::path::Path;

use clap::Parser;
use dnd::character::Character;

// This needs to be reworked
#[derive(Parser, Debug)]
#[command(version, about)]
pub(crate) struct Config {
    pub path: String,
}
