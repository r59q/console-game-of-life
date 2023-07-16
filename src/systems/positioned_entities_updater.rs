use crate::components::position::Position;
use crate::components::ui_entity_flag::UIEntityFlag;
use crate::resources::positioned_entities::PositionedEntities;
use bevy_ecs::prelude::{Entity, Query};
use bevy_ecs::query::Without;
use bevy_ecs::system::{Commands, ResMut};

pub fn positioned_entities_updater(
    query: Query<(Entity, &Position, Without<UIEntityFlag>)>,
    mut positioned_entities: ResMut<PositionedEntities>,
) {
    positioned_entities.clear();
    for (queried) in &query {
        let (entity, position, _) = queried;
        positioned_entities.put_entity(entity, *position)
    }
}
