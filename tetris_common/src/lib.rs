use bevy::prelude::{App, Plugin, Startup, Update};

use crate::systems::{collision_check, setup_board};

pub mod components;
pub mod events;
pub mod systems;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_board)
            .add_systems(Update, collision_check);
    }
}
