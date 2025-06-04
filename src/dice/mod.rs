use rand::{self, Rng};
use serde::{Deserialize, Serialize};
pub mod consts;

#[derive(Debug, Serialize, Deserialize)]
pub struct Dice(u8);

pub trait Rollable {
    fn roll_skill(&self, advantage: u8, disadvantage: u8) -> u8;
    fn roll_amount(&self, times: u8) -> usize;
}

impl Rollable for Dice {
    fn roll_skill(&self, advantage: u8, disadvantage: u8) -> u8 {
        let (cmpfn, mut num): (fn(u8, u8) -> u8, _) = if advantage >= disadvantage {
            (std::cmp::max, advantage - disadvantage)
        } else {
            (std::cmp::min, disadvantage - advantage)
        };

        let range = 1..=self.0;
        let mut rand = rand::rng();

        let mut val = rand.random_range(range.clone());
        while num > 0 {
            let new = rand.random_range(range.clone());
            val = cmpfn(val, new);
            num -= 1;
        }

        val
    }

    fn roll_amount(&self, times: u8) -> usize {
        let range = 1..=self.0 as usize;
        let mut rand = rand::rng();

        (0..times).map(|_| rand.random_range(range.clone())).sum()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::dice::Rollable;
    use crate::dice::consts::*;
    use approx::*;

    #[test]
    fn roll_avg() {
        let dice = [D4, D6, D8, D10, D12, D20, D100];
        let expt = [2.5, 3.5, 4.5, 5.5, 6.5, 10.5, 50.5];

        for (i, die) in dice.into_iter().enumerate() {
            let avg_many = (0..0xffff)
                .map(|_| die.roll_skill(0, 0) as u128)
                .sum::<u128>() as f64
                / 0xffff as f64;

            assert_relative_eq!(avg_many, expt[i], max_relative = 0.05);
        }
    }

    #[test]
    fn roll_adv() {
        let avg_many = (0..0xffff)
                .map(|_| D20.roll_skill(1, 0) as u128)
                .sum::<u128>() as f64
                / 0xffff as f64;
        assert_relative_eq!(avg_many, 13.825, max_relative = 0.05);
    }

    #[test]
    fn roll_dis() {
        let avg_many = (0..0xffff)
                .map(|_| D20.roll_skill(0, 1) as u128)
                .sum::<u128>() as f64
                / 0xffff as f64;
        assert_relative_eq!(avg_many, 7.1575, max_relative = 0.05);
    }

    #[test]
    fn roll_avg_multiple() {
        let dice = [D4, D6, D8, D10, D12, D20, D100];
        let expt  = [2.5, 3.5, 4.5, 5.5, 6.5, 10.5, 50.5].map(|val| val * 6.0);

        for (i, die) in dice.into_iter().enumerate() {
            let avg_many = (0..0xffff)
                .map(|_| die.roll_amount(6) as f64)
                .sum::<f64>()
                / 0xffff as f64;

            assert_relative_eq!(avg_many, expt[i], max_relative = 0.05);
        }
    }
}
