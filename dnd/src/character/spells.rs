use serde::{Deserialize, Serialize};

use crate::{cost::Cost, time::Time};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpellLevel {
    Cantrip,
    L1,    
    L2,    
    L3,    
    L4,    
    L5,    
    L6,    
    L7,    
    L8,    
    L9,    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Spell<'a> {
    name: &'a str,
    descroption: &'a str,
    level: SpellLevel,
    casting_time: Time,
    duration: Time,
    range: SpellRange,
    effect: MagicEffect,
    components: Components<'a>,
    school: School,
    concentration: bool,
    // target: todo!() is this really necessary?
}

// TODO
#[derive(Debug, Serialize, Deserialize)]
pub enum MagicEffect {
    
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SpellRange {
    Touch,
    SelfRange,
    Range(usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum School {
    Transmutation,
    Divination,
    Evocation,
    Abjuration,
    Necromancy,
    Enchantment,
    Illusion,
    Conjuration
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Components<'a> {
    verbal: bool,
    somatic: bool,
    #[serde(borrow)]
    material: Option<MaterialComponent<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialComponent<'a> {
    components: &'a str,
    cost: Option<Cost>
}
