use serde::{Deserialize, Serialize};

use crate::dice::{Dice, Rollable};

#[derive(Debug, Serialize, Deserialize)]
pub struct Class<'a> {
    pub class_name: &'a str,
    pub hit_dice: Dice,

}
