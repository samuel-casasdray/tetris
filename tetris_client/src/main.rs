use bevy::prelude::*;
use bevy::window::WindowResized;

use crate::board::{BoardCalculator, BoardPoint};
use crate::board_walls::BoardWallsBundle;
use crate::game_settings::GameSettings;

mod block;
mod game_settings;
mod board_walls;
mod board;

fn main() {
    App::new()
        // TODO: initialiser toutes les ressources en premier
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
    let board_calculator = BoardCalculator::new(Vec2::new(window.width() / 2., window.height() / 2.), game_settings.block_size);

    let colors = [
        Color::RED,
        Color::BLUE,
        Color::GREEN,
        Color::YELLOW,
    ];

    for x in 0usize..10usize {
        for y in 0usize..20usize {
            let block = block::BlockBundle::new(
                &board_calculator,
                BoardPoint(x as i32, y as i32),
                colors[(x + y) % colors.len()],
                game_settings.block_size,
            );
            commands.spawn(block);
        }
    }

    commands.insert_resource(board_calculator);
    commands.insert_resource(game_settings);
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
        ..default()
    });
}

// TODO: La position et la taille du wall devraient être géré par le board_calculator
fn setup_walls(
    mut commands: Commands,
    game_settings: Res<GameSettings>,
    board_cal: Res<BoardCalculator>,
) {
    let block_size = game_settings.block_size;
    let (board_width, board_height) = game_settings.board_size;

    let left_wall = BoardWallsBundle::left_wall(
        BoardPoint(-1, 0),
        &board_cal,
        board_height as i32 + 2,
        block_size,
    );
    let right_wall = BoardWallsBundle::right_wall(
        BoardPoint(game_settings.board_size.0 as i32, 0),
        &board_cal,
        board_height as i32,
        block_size,
    );
    let top_wall = BoardWallsBundle::top_wall(
        BoardPoint(-1, game_settings.board_size.1 as i32),
        &board_cal,
        board_width as i32 + 2,
        block_size,
    );
    let bottom_wall = BoardWallsBundle::bottom_wall(
        BoardPoint(-1, -1),
        &board_cal,
        board_width as i32 + 2,
        block_size,
    );

    commands.spawn(left_wall);
    commands.spawn(right_wall);
    commands.spawn(top_wall);
    commands.spawn(bottom_wall);
}
