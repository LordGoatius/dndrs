use std::{fmt::Display, ops::Index};

use serde::{Deserialize, Serialize};

use super::{Ability, Prof};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Skill {
    pub acrobatics: Prof,
    pub stealing: Prof,
    pub stealth: Prof,

    //Str
    pub athletics: Prof,

    // Int
    pub arcana: Prof,
    pub history: Prof,
    pub investigation: Prof,
    pub nature: Prof,
    pub religion: Prof,

    // Wis
    pub animal: Prof,
    pub insight: Prof,
    pub medicine: Prof,
    pub perception: Prof,
    pub survival: Prof,

    // Chr
    pub deception: Prof,
    pub intimidation: Prof,
    pub performance: Prof,
    pub persuasion: Prof,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SkillOption {
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
    Persuasion
}

impl From<SkillOption> for Ability {
    fn from(skill: SkillOption) -> Ability {
        match skill {
            // Dex
            SkillOption::Acrobatics => Ability::Dex,
            SkillOption::Stealing => Ability::Dex,
            SkillOption::Stealth => Ability::Dex,

            //Str
            SkillOption::Athletics => Ability::Str,

            // Con

            // Int
            SkillOption::Arcana => Ability::Int,
            SkillOption::History => Ability::Int,
            SkillOption::Investigation => Ability::Int,
            SkillOption::Nature => Ability::Int,
            SkillOption::Religion => Ability::Int,

            // Wis
            SkillOption::Animal => Ability::Wis,
            SkillOption::Insight => Ability::Wis,
            SkillOption::Medicine => Ability::Wis,
            SkillOption::Perception => Ability::Wis,
            SkillOption::Survival => Ability::Wis,

            // Chr
            SkillOption::Deception => Ability::Chr,
            SkillOption::Intimidation => Ability::Chr,
            SkillOption::Performance => Ability::Chr,
            SkillOption::Persuasion => Ability::Chr,
        }
    }
}

impl Index<SkillOption> for Skill {
    type Output = Prof;

    fn index(&self, index: SkillOption) -> &Self::Output {
        match index {
            // Dex
            SkillOption::Acrobatics => &self.acrobatics,
            SkillOption::Stealing => &self.stealing,
            SkillOption::Stealth => &self.stealth,

            //Str
            SkillOption::Athletics => &self.athletics,

            // Con

            // Int
            SkillOption::Arcana => &self.arcana,
            SkillOption::History => &self.history,
            SkillOption::Investigation => &self.investigation,
            SkillOption::Nature => &self.nature,
            SkillOption::Religion => &self.religion,

            // Wis
            SkillOption::Animal => &self.animal,
            SkillOption::Insight => &self.insight,
            SkillOption::Medicine => &self.medicine,
            SkillOption::Perception => &self.perception,
            SkillOption::Survival => &self.survival,

            // Chr
            SkillOption::Deception => &self.deception,
            SkillOption::Intimidation => &self.intimidation,
            SkillOption::Performance => &self.performance,
            SkillOption::Persuasion => &self.persuasion
        }
    }
}

impl Display for SkillOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Dex
            SkillOption::Acrobatics => write!(f, "Acrobatics"),
            SkillOption::Stealing => write!(f, "Slight of Hand"),
            SkillOption::Stealth => write!(f, "Stealth"),

            //Str
            SkillOption::Athletics => write!(f, "Athletics"),

            // Con

            // Int
            SkillOption::Arcana => write!(f, "Arcana"),
            SkillOption::History => write!(f, "History"),
            SkillOption::Investigation => write!(f, "Investigation"),
            SkillOption::Nature => write!(f, "Nature"),
            SkillOption::Religion => write!(f, "Religion"),

            // Wis
            SkillOption::Animal => write!(f, "Animal"),
            SkillOption::Insight => write!(f, "Insight"),
            SkillOption::Medicine => write!(f, "Medicine"),
            SkillOption::Perception => write!(f, "Perception"),
            SkillOption::Survival => write!(f, "Survival"),

            // Chr
            SkillOption::Deception => write!(f, "Deception"),
            SkillOption::Intimidation => write!(f, "Intimidation"),
            SkillOption::Performance => write!(f, "Performance"),
            SkillOption::Persuasion => write!(f, "Persuasion"),
                }
        
    }
}
