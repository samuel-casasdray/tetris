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
        grid_position.x = parent_grid_pos.x + relative_pos.x;
        grid_position.y = parent_grid_pos.y + relative_pos.y;
    }
}
