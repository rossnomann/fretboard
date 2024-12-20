use crate::tuning::Tuning;

pub const APPLICATION_TITLE: &str = "Fretboard";

#[derive(Debug, Clone)]
pub struct Config {
    pub frets_count: u8,
    pub tuning: Tuning,
}

impl Config {
    const DEFAULT_FRETS_COUNT: u8 = 24;
}

impl Default for Config {
    fn default() -> Self {
        Self {
            frets_count: Self::DEFAULT_FRETS_COUNT,
            tuning: Tuning::default(),
        }
    }
}
