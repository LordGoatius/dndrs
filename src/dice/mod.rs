use rand::{self, Rng};
pub mod consts;

pub enum AdvNorDis {
    Advantage,
    Normal,
    Disadvantage,
}

pub struct Dice(usize);

pub trait Rollable {
    fn roll(&self, times: usize, advantage: AdvNorDis) -> usize;
}

impl Rollable for Dice {
    fn roll(&self, times: usize, advantage: AdvNorDis) -> usize {
        let res = rand::rng().random_range(1..=self.0);
        match advantage {
            AdvNorDis::Advantage => todo!(),
            AdvNorDis::Normal => todo!(),
            AdvNorDis::Disadvantage => todo!(),
        }
        todo!()
    }
}
