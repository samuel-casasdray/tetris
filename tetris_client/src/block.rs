use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Bundle, Color, Component, default, Sprite, Transform};
use bevy::sprite::SpriteBundle;

use crate::pos::Pos;

#[derive(Bundle)]
pub struct BlockBundle {
    id: BlockId,
    position: Pos,
    pub sprite_bundle: SpriteBundle,
}

#[derive(Component)]
pub struct BlockId;

impl BlockBundle {
    pub fn new(
        pos: Pos,
        color: Color,
        block_size: f32,
    ) -> Self {
        let sprite_bundle = SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(
                    Vec2::new(block_size, block_size)
                ),
                anchor: bevy::sprite::Anchor::BottomLeft,
                ..default()
            },
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
            sprite_bundle,
        }
    }
}
