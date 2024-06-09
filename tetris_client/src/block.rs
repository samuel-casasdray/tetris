use bevy::math::{Vec2, Vec3};
use bevy::prelude::{default, Color, Sprite, Transform};
use bevy::sprite::SpriteBundle;

use tetris_common::components::GridPosition;

use crate::board_ui_calculator::BoardUICalculator;

pub fn new_block(
    board_calculator: &BoardUICalculator,
    block_position: GridPosition,
    color: Color,
    block_size: f32,
) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite {
            color,
            custom_size: Some(Vec2::new(block_size, block_size)),
            anchor: bevy::sprite::Anchor::BottomLeft,
            ..default()
        },
        transform: Transform {
            translation: board_calculator
                .window_relative_position(&block_position)
                .extend(0.),
            scale: Vec3::splat(1.),
            ..default()
        },
        ..default()
    }
}
