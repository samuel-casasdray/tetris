use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
static BLOCK_SIZE: f32 = 50.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_game, renderBoard, renderBlocks).chain())
        // .add_systems(Update, renderBlocks)
        .run()
}


#[derive(Clone)]
struct MyColor(String);

#[derive(Component)]
struct Board {
    blocks: Vec<Option<MyColor>>,
}

#[derive(Component)]
struct Shape {}

fn setup_game(mut commands: Commands) {
    let cam_position = boardToScreen(BOARD_WIDTH as f32, BOARD_HEIGHT as f32, None);
    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(cam_position.0 / 2., cam_position.1 / 2., 0.0),
        ..default()
    });

    let mut blocks = vec![None; BOARD_WIDTH * BOARD_HEIGHT];

    blocks[0] = Some(MyColor("FF0000".into()));
    blocks[1] = Some(MyColor("00FF00".into()));
    blocks[2] = Some(MyColor("0000FF".into()));
    blocks[3] = Some(MyColor("FFFF00".into()));
    blocks[4] = Some(MyColor("FF00FF".into()));

    let board = Board {
        blocks,
    };

    commands.spawn(board);
}

fn renderBoard(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let wall_width = 5.;

    let horizontal_bar_width = BLOCK_SIZE * BOARD_WIDTH as f32;
    let vertical_bar_height = BLOCK_SIZE * BOARD_HEIGHT as f32;

    let horizontal_bar = Mesh2dHandle(meshes.add(Rectangle::new(horizontal_bar_width + BLOCK_SIZE, wall_width)));
    let vertical_bar = Mesh2dHandle(meshes.add(Rectangle::new(wall_width, vertical_bar_height + BLOCK_SIZE)));

    let color = Color::hex("FFFFFF").unwrap();

    let top_position = boardToScreen(0., BOARD_HEIGHT as f32, Some(Offset(horizontal_bar_width / 2., BLOCK_SIZE / 2.)));
    let bottom_position = boardToScreen(0., 0., Some(Offset(horizontal_bar_width / 2., BLOCK_SIZE / -2.)));
    let left_position = boardToScreen(0., 0., Some(Offset(BLOCK_SIZE / -2., vertical_bar_height / 2.)));
    let right_position = boardToScreen(BOARD_WIDTH as f32, 0., Some(Offset(BLOCK_SIZE / 2., vertical_bar_height / 2.)));

    let shapes = [
        (horizontal_bar.clone(), top_position.0, top_position.1),
        (horizontal_bar, bottom_position.0, bottom_position.1),
        (vertical_bar.clone(), left_position.0, left_position.1),
        (vertical_bar, right_position.0, right_position.1),
    ];

    for (shape, x, y) in shapes {
        commands.spawn(MaterialMesh2dBundle {
            mesh: shape,
            material: materials.add(color),
            transform: Transform {
                translation: Vec3::new(x, y, 0.),
                scale: Vec3::splat(1.),
                ..default()
            },
            ..default()
        });
    }
}


struct Offset(f32, f32);

// Ne marche pas encore comme je le souhaite
fn boardToScreen(x: f32, y: f32, offset: Option<Offset>) -> (f32, f32) {
    match offset {
        None => (x * BLOCK_SIZE, y * BLOCK_SIZE),
        Some(Offset(x_offset, y_offset)) => (x * BLOCK_SIZE + x_offset, y * BLOCK_SIZE + y_offset),
    }
}

fn renderBlocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Query<&Board>,
) {
    for (index, color) in board.single().blocks.iter().enumerate() {
        if let Some(MyColor(color)) = color {
            let x = (index % BOARD_WIDTH) as f32;
            let y = (index / BOARD_WIDTH) as f32;
            println!("{} {} {}", index, y, BOARD_HEIGHT);
            let shape = Mesh2dHandle(meshes.add(Rectangle::new(BLOCK_SIZE, BLOCK_SIZE)));
            let color = Color::hex(color);

            commands.spawn(MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(color.unwrap()),
                transform: Transform {
                    translation: Vec3::new(x * BLOCK_SIZE, -y * BLOCK_SIZE, 0.),
                    scale: Vec3::splat(1.),
                    ..default()
                },
                ..default()
            });
        }
    }
}
