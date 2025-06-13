use rand::{self, Rng};
use serde::{Deserialize, Serialize};
pub mod consts;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dice {
    pub times: u8,
    pub die: u8
}

pub trait Rollable {
    /// Should only ever be used for a D20
    /// Refactor to return a RollResult which contains all rolls made, adv/dis tracked?
    /// TODO: that
    fn roll_skill(&self, advantage: u8, disadvantage: u8) -> u8;
    /// For rolls such as 1d8 or 10d10
    fn roll_amount(&self) -> usize;
}

impl Rollable for Dice {
    /// Should only ever be used for a D20
    /// Refactor to return a RollResult which contains all rolls made, adv/dis tracked?
    /// TODO: that
    fn roll_skill(&self, advantage: u8, disadvantage: u8) -> u8 {
        let (cmpfn, mut num): (fn(u8, u8) -> u8, _) = if advantage >= disadvantage {
            (std::cmp::max, advantage - disadvantage)
        } else {
            (std::cmp::min, disadvantage - advantage)
        };

        let range = 1..=self.die;
        let mut rand = rand::rng();

        let mut val = rand.random_range(range.clone());
        while num > 0 {
            let new = rand.random_range(range.clone());
            val = cmpfn(val, new);
            num -= 1;
        }

        val
    }

    fn roll_amount(&self) -> usize {
        let range = 1..=self.die as usize;
        let mut rand = rand::rng();

        (0..self.times).map(|_| rand.random_range(range.clone())).sum()
    }
}

impl Dice {
    pub fn upgrade_times_mut(&mut self, upgrade: u8) {
        self.times += upgrade;
    }

    pub fn upgrade_times(self, upgrade: u8) -> Dice {
        Dice {
            times: self.times + upgrade,
            die: self.die
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::dice::Rollable;
    use crate::dice::consts::*;
    use approx::*;

    #[test]
    fn roll_avg() {
        let dice = [SIXD4, SIXD6, SIXD8, SIXD10, SIXD12, SIXD20, SIXD100];
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
        let dice = [SIXD4, SIXD6, SIXD8, SIXD10, SIXD12, SIXD20, SIXD100];
        let expt  = [2.5, 3.5, 4.5, 5.5, 6.5, 10.5, 50.5].map(|val| val * 6.0);

        for (i, die) in dice.into_iter().enumerate() {
            let avg_many = (0..0xffff)
                .map(|_| die.roll_amount() as f64)
                .sum::<f64>()
                / 0xffff as f64;

            assert_relative_eq!(avg_many, expt[i], max_relative = 0.05);
        }
    }

    #[test]
    fn test_roll() {
        println!("{}", D20.roll_skill(0, 0));
    }
}
