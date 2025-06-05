pub mod class;
pub mod error;
pub mod level;
pub mod proficiencies;
pub mod skills;
pub mod stats;
pub mod spells;

use std::{fmt::Display, ops::Index};

use class::Class;
use level::Level;
use proficiencies::Proficiencies;
use serde::{Deserialize, Serialize};
use skills::{Skill, SkillOption};
use stats::Stats;

use crate::{
    dice::{self, Rollable},
    languages::Language,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Character<'a> {
    level: Level,
    stats: Stats,
    skills: Skill,
    saving_throws: SavingThrows,
    speed: Speed,
    #[serde(borrow)]
    class: Vec<Class<'a>>,
    hp: usize,
    proficiencies: Proficiencies<'a>,
    // species: Species,
    languages: Vec<Language<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Speed {
    walking: usize,
    swimming: usize,
    flying: usize,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SavingThrows {
    str: Prof,
    dex: Prof,
    con: Prof,
    int: Prof,
    wis: Prof,
    chr: Prof,
}

impl Index<Ability> for SavingThrows {
    type Output = Prof;

    fn index(&self, index: Ability) -> &Self::Output {
        match index {
            Ability::Str => &self.str,
            Ability::Dex => &self.dex,
            Ability::Con => &self.con,
            Ability::Int => &self.int,
            Ability::Wis => &self.wis,
            Ability::Chr => &self.chr,
        }
    }
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

impl Character<'_> {
    pub fn skill_bonus(&self, skill: SkillOption) -> i8 {
        let prof = self.skills[skill];
        let bonus = self.level.prof_bonus() as i8;
        let ability: Ability = skill.into();
        let modifier = self.stats.calculate_mod(ability);
        (prof as i8 * bonus) + modifier
    }

    pub fn saving_throw(&self, ability: Ability) -> i8 {
        let prof = self.saving_throws[ability];
        let bonus = self.level.prof_bonus() as i8;
        let modifier = self.stats.calculate_mod(ability);
        (prof as i8 * bonus) + modifier
    }

    // TODO: Overrides for observant feat etc
    pub fn passive_skills(&self, skill: SkillOption) -> u8 {
        match skill {
            SkillOption::Perception | SkillOption::Investigation | SkillOption::Insight => {
                let skill_bonus = self.skill_bonus(skill);
                (10 + skill_bonus) as u8
            }
            _ => panic!("{skill} is not a valid passive skill"),
        }
    }

    pub fn roll_skill(&self, skill: SkillOption, advantage: u8, disadvantage: u8) -> u8 {
        let roll = dice::consts::D20.roll_skill(advantage, disadvantage) as i8;
        let skill_bonus = self.skill_bonus(skill);
        (roll + skill_bonus) as u8
    }
}

#[cfg(test)]
pub mod test {
    use crate::{
        character::{
            class::Subclass, proficiencies::Proficiencies, skills::Skill, Character, SavingThrows,
            Speed,
        },
        dice,
        items::{ArmorType, Tool},
        languages::Language,
    };

    use super::{class::Class, level::Level, stats::Stats};

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

        let class = {
            use Level::*;
            vec![
                Class {
                    class_name: "Druid".into(),
                    hit_dice: dice::consts::D8,
                    level: Level::L06,
                    asi: vec![L04, L08, L12, L16, L19],
                    class_features: vec![],
                    subclass: Subclass {
                        subclass_name: "Circle of the Dire Beast",
                        subclass_features: vec![],
                    },
                },
                Class {
                    class_name: "Warlock".into(),
                    hit_dice: dice::consts::D8,
                    level: Level::L01,
                    asi: vec![L04, L08, L12, L16, L19],
                    class_features: vec![],
                    subclass: Subclass {
                        subclass_name: "Warlock of the Thicket and Thorn",
                        subclass_features: vec![],
                    },
                },
            ]
        };

        let saving_throws = SavingThrows {
            str: P::None,
            dex: P::None,
            con: P::None,
            int: P::Proficient,
            wis: P::Proficient,
            chr: P::None,
        };

        let speed = Speed {
            walking: 30,
            swimming: 15,
            flying: 0,
        };

        let hp = 52;

        let proficiencies = Proficiencies {
            armor: vec![ArmorType::Light, ArmorType::Medium],
            tools: vec![Tool::HerbalismKit],
            weapons: vec![],
        };

        let languages = {
            use Language::*;
            vec![Druidic, Common, Elvish, Sylvan]
        };

        let liza = Character {
            level,
            stats,
            skills,
            saving_throws,
            speed,
            class,
            hp,
            proficiencies,
            languages,
        };

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

        println!("{liza:#?}");
    }
}
