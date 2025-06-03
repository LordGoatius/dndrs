use std::ops::{Index, IndexMut};

use serde::{Deserialize, Serialize};

use super::{error::Error, Ability};

static SCORE_TABLE: [i8; 31] = [i8::MIN, -5, -4, -4, -3, -3, -2, -2, -1, -1, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10];

#[derive(Debug, Serialize, Deserialize)]
pub struct Stats([u8; 6]);

impl Stats {
    pub fn try_from(stats: [u8; 6]) -> Result<Stats, Error> {
        stats.iter().map(|x| if *x <= 30 {
                Ok(*x)
            } else {
                Err(Error::InvalidAbilityScoreRange)
            })
        .try_collect::<Vec<u8>>()
        .map(|vec| {
            let arr: [u8; 6] = vec.try_into().unwrap();
            Stats(arr)
        })
 }
}

impl Index<Ability> for Stats {
    type Output = u8;

    fn index(&self, index: Ability) -> &Self::Output {
        &self.0[index as usize]
    }
}


impl IndexMut<Ability> for Stats {
    fn index_mut(&mut self, index: Ability) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

impl Stats {
    pub fn calculate_mod(&self, ability: Ability) -> i8 {
        let score = self[ability];
        SCORE_TABLE[score as usize]
    }
}

#[cfg(test)]
pub mod tests {
    use crate::character::stats::Stats;

    #[test]
    fn test_modifiers() {
        let stats = Stats([11, 12, 13, 14, 15, 16]);

        let stats_ = Stats([9, 7, 5, 18, 20, 22]);

        let check_arr = [0, 1, 1, 2, 2, 3];
        let check_arr_ = [-1, -2, -3, 4, 5, 6];

        use crate::character::Ability;
        for (i, ability) in [Ability::Str, Ability::Dex, Ability::Con, Ability::Int, Ability::Wis, Ability::Chr].into_iter().enumerate() {
            eprintln!("{}, {}, {:?}", stats.calculate_mod(ability), check_arr[i], ability);
            assert_eq!(stats.calculate_mod(ability), check_arr[i]);
        }

        for (i, ability) in [Ability::Str, Ability::Dex, Ability::Con, Ability::Int, Ability::Wis, Ability::Chr].into_iter().enumerate() {
            eprintln!("{}, {}, {:?}", stats_.calculate_mod(ability), check_arr_[i], ability);
            assert_eq!(stats_.calculate_mod(ability), check_arr_[i]);
        }
    }
}
