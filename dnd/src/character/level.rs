use serde::{Deserialize, Serialize};

// Using an enum for level is probably unnecessary, but I think it's reasonably okay here.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    L01,
    L02,
    L03,
    L04,
    L05,
    L06,
    L07,
    L08,
    L09,
    L10,
    L11,
    L12,
    L13,
    L14,
    L15,
    L16,
    L17,
    L18,
    L19,
    L20,
    L21,
    L22,
    L23,
    L24,
    L25,
    L26,
    L27,
    L28,
    L29,
    L30,
}

impl Level {
    pub fn prof_bonus(&self) -> u8 {
        ((*self as u8) / 4) + 2
    }
}

#[cfg(test)]
pub mod test {
    #[test]
    fn prof_bonus() {
        use super::Level::*;
        let level = [
            L01, L02, L03, L04, L05, L06, L07, L08, L09, L10, L11, L12, L13, L14, L15, L16, L17,
            L18, L19, L20, L21, L22, L23, L24, L25, L26, L27, L28, L29, L30,
        ];
        let valid = [2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 9, 9];

        for i in 0..level.len() {
            assert_eq!(level[i].prof_bonus(), valid[i]);
        }
    }
}
