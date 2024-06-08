use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::window::WindowResized;
// use rand::Rng;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_game, renderBoard, renderBlocks).chain())
        .add_systems(Update, on_resize_system)
        .run()
}

fn on_resize_system(
    mut camera2d_bundle: Query<&mut Transform, With<Camera>>,
    mut resize_reader: EventReader<WindowResized>,
    mut block_size: ResMut<BlockSize>,
    mut blocks : Query<&mut Transform, (With<Block>, Without<Camera>)>
) {
    for e in resize_reader.read() {
        camera2d_bundle.single_mut().translation = Vec3::new(e.width / 2., e.height / -2., 0.);
        let old_block_size = block_size.0;
        block_size.0 = get_block_size(e.width, e.height);
        for mut block in blocks.iter_mut() {
            let old_block_scale = block.scale.clone().x;
            let new_scale = old_block_scale * block_size.0 / old_block_size;
            *block = block.with_scale(Vec3::new(new_scale, new_scale, new_scale));
        }
    }
}

fn get_block_size(width: f32, height: f32) -> f32{
    let width = width * 0.4;
    let height = height * 0.9;
    if width / BOARD_WIDTH as f32 > height / BOARD_HEIGHT as f32 {
        height / BOARD_HEIGHT as f32
    } else {
        width / BOARD_WIDTH as f32
    }
}

#[derive(Clone)]
struct MyColor(Color);

#[derive(Component)]
struct Board {
    blocks: Vec<Option<MyColor>>,
}

#[derive(Component)]
struct Block;

#[derive(Resource)]
struct BlockSize(f32);

fn setup_game(mut commands: Commands, window: Query<&Window>){
    let window = window.single();
    let size = get_block_size(window.width(), window.height());
    commands.insert_resource(BlockSize(size));

    let cam_position = boardToScreen(BOARD_WIDTH as f32, BOARD_HEIGHT as f32, None, size);
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(cam_position.0 / 2., cam_position.1 / 2., 0.0),
        ..default()
    });

    // let mut rng = rand::thread_rng();
    // let mut blocks = Vec::<Option<MyColor>>::new();
    //
    // for i in 0..(BOARD_WIDTH*BOARD_HEIGHT) {
    //     blocks.push(Some(MyColor(Color::rgb(rng.gen(), rng.gen(), rng.gen()))))
    // }

    let mut blocks = vec![None; BOARD_WIDTH * BOARD_HEIGHT];

    blocks[0] = Some(MyColor(Color::hex(String::from("FF0000")).unwrap()));
    blocks[1] = Some(MyColor(Color::hex(String::from("00FF00")).unwrap()));
    blocks[2] = Some(MyColor(Color::hex(String::from("0000FF")).unwrap()));
    blocks[3] = Some(MyColor(Color::hex(String::from("FFFF00")).unwrap()));
    blocks[4] = Some(MyColor(Color::hex(String::from("FF00FF")).unwrap()));

    let board = Board {
        blocks,
    };

    commands.spawn(board);
}

fn renderBoard(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    block_size_res: Res<BlockSize>
) {
    let block_size = block_size_res.0;
    let wall_width = 5.;

    let horizontal_bar_width = block_size * BOARD_WIDTH as f32;
    let vertical_bar_height = block_size * BOARD_HEIGHT as f32;

    let horizontal_bar = Mesh2dHandle(meshes.add(Rectangle::new(horizontal_bar_width + block_size, wall_width)));
    let vertical_bar = Mesh2dHandle(meshes.add(Rectangle::new(wall_width, vertical_bar_height + block_size)));

    let color = Color::hex("FFFFFF").unwrap();

    let top_position = boardToScreen(0., BOARD_HEIGHT as f32, Some(Offset(horizontal_bar_width / 2., block_size / 2.)), block_size);
    let bottom_position = boardToScreen(0., 0., Some(Offset(horizontal_bar_width / 2., block_size / -2.)), block_size);
    let left_position = boardToScreen(0., 0., Some(Offset(block_size / -2., vertical_bar_height / 2.)), block_size);
    let right_position = boardToScreen(BOARD_WIDTH as f32, 0., Some(Offset(block_size / 2., vertical_bar_height / 2.)), block_size);

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
fn boardToScreen(x: f32, y: f32, offset: Option<Offset>, block_size: f32) -> (f32, f32) {
    match offset {
        None => (x * block_size, y * block_size),
        Some(Offset(x_offset, y_offset)) => (x * block_size + x_offset, y * block_size + y_offset),
    }
}

fn renderBlocks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: Query<&Board>,
    block_size: Res<BlockSize>
){
    for (index, color) in board.single().blocks.iter().enumerate(){
        if let Some(MyColor(color)) = color {
            let x = (index % BOARD_WIDTH) as f32;
            let y = (index / BOARD_WIDTH) as f32;
            let shape = Mesh2dHandle(meshes.add(Rectangle::new(block_size.as_ref().0, block_size.as_ref().0)));

            commands.spawn((MaterialMesh2dBundle {
                mesh: shape,
                material: materials.add(*color),
                transform: Transform {
                    translation: Vec3::new(x*block_size.as_ref().0, -y * block_size.as_ref().0, 0.),
                    scale: Vec3::splat(1.),
                    ..default()
                },
                ..default()
            }, Block));
        }
    }
}
