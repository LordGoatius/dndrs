use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Ability scores must be between 0..=30")]
    InvalidAbilityScoreRange,
}
