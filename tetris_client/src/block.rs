use bevy::asset::{Handle};
use bevy::math::Vec3;
use bevy::prelude::{Bundle, Component, default, Transform};
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle};

#[derive(Component)]
pub struct Pos(pub f32, pub f32);

#[derive(Bundle)]
pub struct BlockBundle {
    id: BlockId,
    position: Pos,
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Component)]
pub struct BlockId;

impl BlockBundle {
    pub fn new(
        pos: Pos,
        mesh_handle: Mesh2dHandle,
        material_handle: Handle<ColorMaterial>,
        block_size: f32,
    ) -> Self {
        let mesh = MaterialMesh2dBundle {
            mesh: mesh_handle,
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(pos.0 * block_size, pos.1 * block_size, 0.),
                scale: Vec3::splat(1.),
                ..default()
            },
            ..default()
        };

        Self {
            id: BlockId,
            position: pos,
            mesh,
        }
    }

    pub fn update_position(&mut self, position: Pos) {
        self.position = position;
        self.mesh.transform = Transform {
            translation: Vec3::new(self.position.0, self.position.1, 0.),
            scale: Vec3::splat(1.),
            ..default()
        }
    }
}
