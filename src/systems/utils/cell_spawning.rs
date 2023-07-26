use bevy_ecs::system::Commands;

use crate::{
    components::position::Position, prefabs::Prefabs,
    resources::positioned_entities::PositionedEntities,
};

/// Helper function to spawn cell, adding it to the PositionedEntities struct.
pub fn spawn_cell_at(
    target_pos: (i32, i32),
    commands: &mut Commands,
    positioned_entities: &mut PositionedEntities,
) {
    let new_cell_pos_component = Position {
        x: target_pos.0 as f64,
        y: target_pos.1 as f64,
    };
    let placed_entity =
        Prefabs::CELL_WITHOUT_POSITION(&mut commands.spawn().insert(new_cell_pos_component));
    positioned_entities.put_entity(placed_entity, new_cell_pos_component);
}

/// Helper function to despawn a cell, removing it from the PositionedEntities struct.
pub fn despawn_cell_at(
    target_pos: (i32, i32),
    commands: &mut Commands,
    positioned_entities: &mut PositionedEntities,
) {
    commands
        .entity(*positioned_entities.get(&target_pos).unwrap())
        .despawn();
    positioned_entities.remove_entity(target_pos);
}
