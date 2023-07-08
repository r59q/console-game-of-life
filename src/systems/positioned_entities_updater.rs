use crate::components::position::Position;
use crate::resources::positioned_entities::PositionedEntities;
use bevy_ecs::prelude::{Entity, Query};
use bevy_ecs::system::{Commands, ResMut};

pub fn positioned_entities_updater(
    query: Query<(Entity, &Position)>,
    mut positioned_entities: ResMut<PositionedEntities>,
) {
    positioned_entities.clear();
    for (queried) in &query {
        let (entity, position) = queried;
        positioned_entities.put_entity(entity, *position)
    }
}
