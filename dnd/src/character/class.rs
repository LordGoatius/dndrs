use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::dice::Dice;

use super::{level::Level, spells::{SpellLevel, Spell}};

#[derive(Debug, Serialize, Deserialize)]
pub struct Class<'a> {
    pub class_name: &'a str,
    pub subclass: Subclass<'a>,
    pub hit_dice: Dice,
    pub level: Level,
    pub asi: Vec<Level>,
    pub class_features: Vec<ClassFeature<'a>>,
    pub class_type: ClassType<'a>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subclass<'a> {
    pub subclass_name: &'a str,
    pub subclass_features: Vec<ClassFeature<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClassFeature<'a> {
    level: Level,
    description: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClassType<'a> {
    Spellcasting {
        cantrips: [u8; 20],
        spell_slot_table: BTreeMap<(Level, SpellLevel), u8>,
        #[serde(borrow)]
        spell_list: Vec<Spell<'a>>,
    },
    PactMagic {
        cantrips: [u8; 20],
        spell_slot_list: [SpellLevel; 20]
    },
    // Ignoring martial for now because I play a druid in my campaign lmao
    Martial {
        
    },
}
