use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct BlockCollisionEvent;

#[derive(Event)]
pub struct WallCollisionEvent(Entity);
