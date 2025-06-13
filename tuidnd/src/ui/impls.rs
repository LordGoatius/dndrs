use dnd::character::{Ability, Character};
use ratatui::style::{Color, Style};
use ratatui::symbols::border;
use ratatui::text::{Text, ToLine};
use ratatui::widgets::{Block, Paragraph, Widget};
use derive_more::Deref;

#[derive(Deref, Debug)]
#[deref(forward)]
pub(super) struct RenderCharacter<'a>(pub(super) &'a Character<'a>);

impl Widget for &RenderCharacter<'_> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let char= format!(" {} ", self.name);
        let title = char.to_line();
        let abilities = {
            use Ability::*;
            let abilities = [Str, Dex, Con, Int, Wis, Chr];
            abilities
                .map(|a| format!("{}: {}", a, self.0.stats[a]))
                .into_iter()
                .intersperse(" ".into())
                .collect::<String>()
        };

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(abilities)
            .title_style(Style::new().fg(Color::DarkGray))
            .border_style(Style::new().fg(Color::Rgb(0x33, 0xa0, 0x33)))
            .border_set(border::THICK);

        let counter_text = Text::from("This is some text");

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
