use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Bundle, Color, Component, default, Sprite, Transform};
use bevy::sprite::SpriteBundle;

use crate::board::{BoardCalculator, BoardPoint};

#[derive(Bundle)]
pub struct BlockBundle {
    id: BlockId,
    position: BoardPoint,
    pub sprite_bundle: SpriteBundle,
}

#[derive(Component)]
pub struct BlockId;

impl BlockBundle {
    pub fn new(
        board_calculator: &BoardCalculator,
        board_position: BoardPoint,
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
                translation: board_calculator.window_relative_position(&board_position).extend(0.),
                scale: Vec3::splat(1.),
                ..default()
            },
            ..default()
        };

        Self {
            id: BlockId,
            position: board_position,
            sprite_bundle,
        }
    }
}
