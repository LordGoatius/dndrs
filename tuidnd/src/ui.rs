use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Borders};

use crate::tui::Frame;
use crate::App;

pub mod impls;
pub use impls::*;

pub fn render(app: &mut App, f: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
                     Constraint::Percentage(12),
                     Constraint::Percentage(76),
                     Constraint::Percentage(12),
        ])
        .split(f.area());
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
                     Constraint::Percentage(20),
                     Constraint::Percentage(60),
                     Constraint::Percentage(20),
        ])
        .split(layout[1]);


    f.render_widget(&RenderCharacter(&app.character), layout[0]);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(Color::Rgb(0x33, 0xa0, 0x33)))
            .title(" Board ")
            .title_style(Style::new().fg(Color::DarkGray))
            .title_alignment(Alignment::Center), cols[1]);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(Color::Rgb(0x33, 0xa0, 0x33)))
            .title(" Left ")
            .title_style(Style::new().fg(Color::DarkGray))
            .title_alignment(Alignment::Center), cols[0]);
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::new().fg(Color::Rgb(0x33, 0xa0, 0x33)))
            .title(" Right ")
            .title_style(Style::new().fg(Color::DarkGray))
            .title_alignment(Alignment::Center), cols[2]);
    f.render_widget(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded)
            .border_style(Style::new().fg(Color::Rgb(0x33, 0xa0, 0x33)))
            .title(" Bottom ")
            .title_style(Style::new().fg(Color::DarkGray))
            .title_alignment(Alignment::Center), layout[2]);
    // f.render_widget(&app.character, f.area());
}
