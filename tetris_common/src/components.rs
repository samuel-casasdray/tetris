use std::fmt::Display;

use bevy::prelude::Component;

/// Particularly used to create a fake shape to test collision against
#[derive(Debug, Component)]
pub struct Fake;

#[derive(Component)]
pub struct ScoreText;

#[derive(Debug, Component, Default)]
pub struct Score {
    pub score: u64,
}

impl Score {
    pub fn new() -> Self {
        Self { score: 0 }
    }
    pub fn add_score_line(&mut self, level: u8, lines: u8) {
        self.score += match lines {
            0 => 0,
            1 => 40,
            2 => 100,
            3 => 300,
            _ => 1200,
        } * (level + 1) as u64;
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Score : {}", self.score)
    }
}
