use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Cost {
    CP(usize),
    SP(usize),
    EP(usize),
    GP(usize),
    PP(usize)
}
