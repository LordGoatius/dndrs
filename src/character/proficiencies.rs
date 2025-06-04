use serde::{Deserialize, Serialize};

use crate::items::{Armor, Tool, Weapon};

#[derive(Debug, Serialize, Deserialize)]
pub struct Proficiencies<'a> {
    #[serde(borrow)]
    armor: Vec<Armor<'a>>,
    #[serde(borrow)]
    tools: Vec<Tool<'a>>,
    #[serde(borrow)]
    weapons: Vec<Weapon<'a>>,
}
