use bevy::prelude::{App, Plugin};

mod systems;
mod components;
mod events;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
    }
}
