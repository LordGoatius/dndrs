use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Cost {
    CP(usize),
    SP(usize),
    EP(usize),
    GP(usize),
    PP(usize)
}
