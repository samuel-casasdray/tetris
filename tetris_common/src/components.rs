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
pub struct Shape;

/// An entity controlled by the user running the app
#[derive(Component)]
pub struct Owned;

/// Particularly used to create a fake shape to test collision against 
#[derive(Component)]
pub struct Fake;

#[derive(Component)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}
