use serde::{Deserialize, Serialize};

use crate::cost::Cost;
pub mod consts;

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
    level: SpellLevel,
    school: School,
    casting_time: &'a str,
    // TODO: use a time unit
    // casting_time: Time,
    range: &'a str,
    duration: &'a str,
    components: Components<'a>,
    description: &'a str,
    // effect: MagicEffect<'a>,
}

// TODO
// #[derive(Debug, Serialize, Deserialize)]
// pub enum MagicEffect<'a> {
//     AttackRoll(AttackRoll),
//     // DC is calculated using spell save DC
//     SavingThrow(Ability),
//     Misc(&'a str)
// }

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
