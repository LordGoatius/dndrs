pub mod class;
pub mod error;
pub mod level;
pub mod skills;
pub mod stats;

use std::fmt::Display;

use class::Class;
use level::Level;
use serde::{Deserialize, Serialize};
use skills::{Skill, SkillOption};
use stats::Stats;

#[derive(Debug, Serialize, Deserialize)]
pub struct Character {
    level: Level,
    stats: Stats,
    skills: Skill,
    class: Vec<Class>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Ability {
    Str,
    Dex,
    Con,
    Int,
    Wis,
    Chr,
}

impl Display for Ability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ability::Str => write!(f, "Str"),
            Ability::Dex => write!(f, "Dex"),
            Ability::Con => write!(f, "Con"),
            Ability::Int => write!(f, "Int"),
            Ability::Wis => write!(f, "Wis"),
            Ability::Chr => write!(f, "Chr"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Prof {
    None,
    Proficient,
    Expert,
}

impl Character {
    pub fn skill_bonus(&self, skill: SkillOption) -> i8 {
        let prof = self.skills[skill];
        println!("Prof: {prof:?}");
        let bonus = self.level.prof_bonus() as i8;
        println!("Prof Bonus: {bonus}");
        let ability: Ability = skill.into();
        let modifier = self.stats.calculate_mod(ability);
        println!("{ability} Mod: {modifier}");
        (prof as i8 * bonus) + modifier
    }
}

#[cfg(test)]
pub mod test {
    use crate::character::{skills::Skill, Character};

    use super::{
        class::{Class, ClassOption},
        level::Level,
        stats::Stats,
    };

    #[test]
    fn char_debug() {
        let level = Level::L07;
        let stats = Stats::try_from([12, 14, 14, 12, 20, 8]).unwrap();
        use super::Prof as P;
        let skills = Skill {
            acrobatics: P::None,
            stealing: P::None,
            stealth: P::None,
            athletics: P::None,
            arcana: P::Proficient,
            history: P::None,
            investigation: P::None,
            nature: P::Proficient,
            religion: P::None,
            animal: P::Proficient,
            insight: P::None,
            medicine: P::Proficient,
            perception: P::Proficient,
            survival: P::Proficient,
            deception: P::None,
            intimidation: P::None,
            performance: P::None,
            persuasion: P::None,
        };
        let class = vec![Class {
            class: ClassOption::Druid,
        }];

        let liza = Character {
            level,
            stats,
            skills,
            class,
        };

        println!("{liza:#?}");

        use super::skills::SkillOption::*;

        let skills = [
            // Dex
            Acrobatics,
            Stealing,
            Stealth,
            //Str
            Athletics,
            // Con

            // Int
            Arcana,
            History,
            Investigation,
            Nature,
            Religion,
            // Wis
            Animal,
            Insight,
            Medicine,
            Perception,
            Survival,
            // Chr
            Deception,
            Intimidation,
            Performance,
            Persuasion,
        ];

        let values = [2, 2, 2, 1, 4, 1, 1, 4, 1, 8, 5, 8, 8, 8, -1, -1, -1, -1];

        for i in 0..skills.len() {
            let skill = skills[i];
            println!("{skill}");
            assert_eq!(liza.skill_bonus(skill), values[i]);
        }
    }
}
