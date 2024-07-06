use bevy::prelude::Event;

use crate::board::components::RelativeGridPosition;
use crate::tetromino::shapes::Shape;

#[derive(Debug, Event)]
pub struct BlockCollisionEvent;

#[derive(Debug, Event)]
pub enum MovementEvent {
    Right,
    Left,
    Down,
    RotationRight,
    RotationLeft,
}

#[derive(Debug, Event)]
pub struct TetrominoSpawnEvent {
    pub shape: Shape,
    pub rotation: u8,
}
#[derive(Debug, Event, Default)]
pub struct TetrominoMovementEvent {
    pub relative_position: Option<RelativeGridPosition>,
    pub rotation: Option<u8>,
}

#[derive(Debug, Event, Default)]
pub struct TetrominoMovementCheckedEvent {
    pub relative_position: Option<RelativeGridPosition>,
    pub rotation: Option<u8>,
}
