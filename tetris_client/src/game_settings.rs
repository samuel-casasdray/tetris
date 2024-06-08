use bevy::prelude::{Resource};

const DEFAULT_BOARD_WIDTH: usize = 10;
const DEFAULT_BOARD_HEIGHT: usize = 20;
const DEFAULT_BLOCK_SIZE: f32 = 50.;

#[derive(Resource, Clone)]
pub struct GameSettings {
    pub block_size: f32,
    pub board_size: (f32, f32),
}

impl Default for GameSettings{
    fn default() -> Self {
        Self{
            board_size: (DEFAULT_BOARD_WIDTH as f32, DEFAULT_BOARD_HEIGHT as f32),
            block_size: DEFAULT_BLOCK_SIZE,
        }
    }
}
