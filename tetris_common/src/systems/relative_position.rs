use bevy::prelude::{Entity, Parent, Query, With};

use crate::components::{GridPosition, RelativeGridPosition};

/// This system update all chidrens GridPosition relative to their parent and their RelativePosition component.
/// Carefull this system only work at 1 level deep
pub fn relative_position_system(
    positions: Query<(Entity, &RelativeGridPosition, &Parent), With<GridPosition>>,
    mut grid_positions: Query<&mut GridPosition>,
) {
    let entities_with_rel_pos_grid_pos: Vec<_> =
        positions
            .iter()
            .map(|(entity, relative_pos, parent)| {
                (
                    entity,
                    relative_pos,
                    grid_positions.get(parent.get()).expect(
                        "To use a relative position the parent must have a GridPosition component",
                    ).clone(),
                )
            })
            .collect();

    for (entity, relative_pos, parent_grid_pos) in entities_with_rel_pos_grid_pos {
        let mut grid_position = grid_positions.get_mut(entity).unwrap();
        *grid_position = get_grid_position(&relative_pos, &parent_grid_pos);
    }
}

pub fn get_grid_position(
    relative_position: &RelativeGridPosition,
    parent_position: &GridPosition,
) -> GridPosition {
    GridPosition::from((
        parent_position.x + relative_position.x,
        parent_position.y + relative_position.y,
    ))
}
