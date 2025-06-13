use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::cost::Cost;
pub mod consts;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    description: Cow<'a, str> ,
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Components<'a> {
    verbal: bool,
    somatic: bool,
    #[serde(borrow)]
    material: Option<MaterialComponent<'a>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MaterialComponent<'a> {
    components: &'a str,
    cost: Option<Cost>
}

#[cfg(test)]
pub mod tests {
    use core::panic;
    use std::borrow::Cow;

    use serde::{Deserialize, Serialize};

    use super::Spell;

    #[test]
    fn test_ron_spells() {
        // #[derive(Debug, Serialize, Deserialize)]
        // struct Description<'a> {
        //     description: Cow<'a, str>,
        // }

        // let description  = crate::character::spells::consts::WISH.description;
        // let description = Description { description };

        let spell = crate::character::spells::consts::WISH;

        let ron = ron::to_string(&spell).unwrap();
        let spell_ron: Spell = ron::from_str(&ron).unwrap();

        // let ron_desc = ron::to_string(&description).unwrap();
        // println!("{ron_desc}");
        // let desc_ron: Description = ron::from_str(&ron_desc).unwrap();
    }
}
