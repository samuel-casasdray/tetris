use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::board_walls::BoardWallsBundle;
use crate::game_settings::GameSettings;
use crate::pos::Pos;

mod block;
mod game_settings;
mod board_walls;
mod pos;
mod board;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_game, setup_walls).chain())
        .add_systems(Update, on_resize_system)
        .run()
}

fn on_resize_system(
    mut camera2d_bundle: Query<&mut Transform, With<Camera>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for window in resize_reader.read() {
        camera2d_bundle.single_mut().translation = Vec3::new(window.width / 2., window.height / 2., 0.);
    }
}

fn setup_game(
    mut commands: Commands,
    window: Query<&Window>,
) {
    let window = window.single();
    let game_settings = GameSettings {
        block_size: 50.,
        ..default()
    };


    let colors = [
        Color::RED,
        Color::BLUE,
        Color::GREEN,
        Color::YELLOW,
    ];

    for x in 0..10 {
        for y in 0..20 {
            let block = block::BlockBundle::new(
                Pos(x as f32, y as f32),
                colors[(x + y) % colors.len()],
                game_settings.block_size,
            );
            commands.spawn(block);
        }
    }

    commands.insert_resource(game_settings);
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
        ..default()
    });
}

fn setup_walls(
    mut commands: Commands,
    game_settings: Res<GameSettings>,
) {
    let block_size = game_settings.block_size;
    let (board_width, board_height) = game_settings.board_size;

    let left_wall = BoardWallsBundle::left_wall(
        Pos(-1., 0.),
        block_size,
        board_height + 2.,
        block_size,
    );
    let right_wall = BoardWallsBundle::right_wall(
        Pos(game_settings.board_size.0, 0.),
        block_size,
        board_height,
        block_size,
    );
    let top_wall = BoardWallsBundle::top_wall(
        Pos(-1., game_settings.board_size.1),
        block_size,
        board_width + 2.,
        block_size,
    );
    let bottom_wall = BoardWallsBundle::bottom_wall(
        Pos(-1., -1.),
        block_size,
        board_width + 2.,
        block_size,
    );

    commands.spawn(left_wall);
    commands.spawn(right_wall);
    commands.spawn(top_wall);
    commands.spawn(bottom_wall);
}
