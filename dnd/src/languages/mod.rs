use derive_more::{Debug, Display};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Copy, Clone, Serialize, Deserialize)]
pub enum Language<'a> {
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Halfling,
    Orc,
    #[display("The language of the Demons")]
    Abyssal,
    Celestial,
    Common,
    Draconic,
    Kraul,
    Loxodon,
    Merfolk,
    Minotaur,
    Sphinx,
    #[display("The language of the Fey")]
    Sylvan,
    Vedalken,
    #[display("The language of the Abberations")]
    #[debug("Deep Speech")]
    DeepSpeech,
    Infernal,
    Primodial,
    Undercommon,
    Aarakocra,
    #[display("The secret language of the Druids")]
    Druidic,
    Gith,
    #[debug("Thieves' Cant")]
    #[display("The secret language of Rogues and Criminals")]
    ThievesCant,
    #[debug("{_0}")]
    #[display("{_1}")]
    Custom(&'a str, &'a str),
}

#[cfg(test)]
pub mod tests {
    use super::Language::{self, *};

    static LANGS: [Language<'static>; 27] = [
        Dwarvish,
        Elvish,
        Giant,
        Gnomish,
        Goblin,
        Halfling,
        Orc,
        Abyssal,
        Celestial,
        Common,
        Draconic,
        Kraul,
        Loxodon,
        Merfolk,
        Minotaur,
        Sphinx,
        Sylvan,
        Vedalken,
        DeepSpeech,
        Infernal,
        Primodial,
        Undercommon,
        Aarakocra,
        Druidic,
        Gith,
        ThievesCant,
        Custom("Custom Lang", "Custom Lang Description")
    ];

    #[test]
    fn print_languages() {
        for lang in LANGS {
            println!("{:?}: {}", lang, lang);
        }
    }
}
