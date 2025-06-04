use serde::{Deserialize, Serialize};

use crate::items::{ArmorType, Tool, Weapon};

#[derive(Debug, Serialize, Deserialize)]
pub struct Proficiencies<'a> {
    pub armor: Vec<ArmorType>,
    #[serde(borrow)]
    pub tools: Vec<Tool<'a>>,
    #[serde(borrow)]
    pub weapons: Vec<Weapon<'a>>,
}
