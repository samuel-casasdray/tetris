use bevy::prelude::{App, Plugin};

pub mod systems;
pub mod components;
pub mod events;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
    }
}
