use bevy::asset::Handle;
use bevy::math::{Vec2};
use bevy::prelude::{Bundle, ColorMaterial, default, Transform};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use crate::pos::Pos;

const WALL_WIDTH: f32 = 25.;

#[derive(Bundle)]
pub struct BoardWallsBundle {
    position: Pos,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}


impl BoardWallsBundle {
    pub fn bottom_wall(
        pos: Pos,
        mesh_handle: Mesh2dHandle,
        material_handle: Handle<ColorMaterial>,
        block_size: f32,
        wall_size: f32,
    ) -> Self {
        let mesh = get_wall_mesh(
            mesh_handle,
            material_handle,
            Vec2 { x: 0., y: 0. },
            Vec2 { x: wall_size * block_size, y: WALL_WIDTH },
        );

        Self {
            position: pos,
            mesh,
        }
    }
    pub fn top_wall(
        pos: Pos,
        mesh_handle: Mesh2dHandle,
        material_handle: Handle<ColorMaterial>,
        block_size: f32,
        wall_size: f32,
    ) -> Self {
        let mesh = get_wall_mesh(
            mesh_handle,
            material_handle,
            Vec2 { x: 0., y: wall_size * block_size },
            Vec2 { x: wall_size * block_size, y: WALL_WIDTH },
        );

        Self {
            position: pos,
            mesh,
        }
    }

    pub fn left_wall(
        pos: Pos,
        mesh_handle: Mesh2dHandle,
        material_handle: Handle<ColorMaterial>,
        block_size: f32,
        wall_size: f32,
    ) -> Self {
        let mesh = get_wall_mesh(
            mesh_handle,
            material_handle,
            Vec2 { x: wall_size * block_size / -2., y: 0. },
            Vec2 { x: WALL_WIDTH, y: wall_size * block_size },
        );

        Self {
            position: pos,
            mesh,
        }
    }

    pub fn right_wall(
        pos: Pos,
        mesh_handle: Mesh2dHandle,
        material_handle: Handle<ColorMaterial>,
        block_size: f32,
        wall_size: f32,
    ) -> Self {
        let mesh = get_wall_mesh(
            mesh_handle,
            material_handle,
            Vec2 { x: wall_size * block_size / 2., y: 0. },
            Vec2 { x: WALL_WIDTH, y: wall_size * block_size },
        );

        Self {
            position: pos,
            mesh,
        }
    }
}

fn get_wall_mesh(
    mesh_handle: Mesh2dHandle,
    material_handle: Handle<ColorMaterial>,
    position: Vec2,
    scale: Vec2,
) -> MaterialMesh2dBundle<ColorMaterial> {
    MaterialMesh2dBundle {
        mesh: mesh_handle,
        material: material_handle,
        transform: Transform {
            translation: position.extend(0.),
            scale: scale.extend(1.),
            ..default()
        },
        ..default()
    }
}