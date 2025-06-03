use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    pub class_name: String,
    pub hit_dice: Dice::D6;
}
