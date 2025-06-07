use derive_more::Display;
use serde::{Deserialize, Serialize};

use crate::dice::Dice;

// TODO
#[derive(Debug, Display, Serialize, Deserialize)]
pub enum DamageType {  
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttackRoll {
    dice: Dice,
    damage_type: DamageType,
}
