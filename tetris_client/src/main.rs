use bevy::prelude::*;
use bevy::window::WindowResized;

use tetris_common::CommonPlugin;

use crate::board_ui_calculator::{BoardPoint, BoardUICalculator};
use crate::board_walls::{BoardWall, BoardWallsBundle};
use crate::game_settings::GameSettings;

mod block;
mod game_settings;
mod board_walls;
mod board_ui_calculator;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CommonPlugin)
        .add_systems(Startup, (setup_resources, setup_game, setup_walls).chain())
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

fn setup_resources(
    mut commands: Commands,
    window: Query<&Window>,
) {
    let window = window.single();
    let game_settings = GameSettings {
        block_size: 50.,
        ..default()
    };
    let board_calculator = BoardUICalculator::new(
        Vec2::new(window.width() / 2., window.height() / 2.),
        game_settings.block_size,
        game_settings.board_size.1,
        game_settings.board_size.0,
    );

    commands.insert_resource(board_calculator);
    commands.insert_resource(game_settings);
}

fn setup_game(
    mut commands: Commands,
    window: Query<&Window>,
    board_calculator: Res<BoardUICalculator>,
    game_settings: Res<GameSettings>,
) {
    let window = window.single();

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

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.0),
        ..default()
    });
}

fn setup_walls(
    mut commands: Commands,
    board_cal: Res<BoardUICalculator>,
) {
    let walls = board_cal.window_relative_board_walls();
    let walls_order = [BoardWall::Top, BoardWall::Bottom, BoardWall::Left, BoardWall::Right];

    for wall in walls.iter().zip(walls_order) {
        let wall_bundle = BoardWallsBundle::new(
            wall.0.0,
            wall.0.1,
            wall.1,
        );
        commands.spawn(wall_bundle);
    }
}
