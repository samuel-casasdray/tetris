use bevy::prelude::Event;

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
