use bevy::prelude::Event;

#[derive(Event)]
pub struct BlockCollisionEvent;

#[derive(Event)]
pub struct WallCollisionEvent;
