use bevy::prelude::Component;

#[derive(Component)]
pub struct Board {
    pub height: usize,
    pub width: usize,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            height: 20,
            width: 10,
        }
    }
}

#[derive(Component)]
pub struct CurrentBoard;

#[derive(Component)]
pub struct Shape;

#[derive(Component)]
pub struct ControlledShape;

#[derive(Component)]
pub struct Block {
    pub x: usize,
    pub y: usize,
}
