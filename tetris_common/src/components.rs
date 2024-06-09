use bevy::prelude::Component;

#[derive(Component)]
pub struct Board;

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
