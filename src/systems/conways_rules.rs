use bevy_ecs::system::{Commands, Res};

use crate::resources::{
    pause_state::{self, PauseState},
    positioned_entities::PositionedEntities,
};

pub fn conways_rules(
    mut commands: Commands,
    positioned_entities: Res<PositionedEntities>,
    pause_state: Res<PauseState>,
) {
    if pause_state.is_paused() {
        return;
    }

    let entities = positioned_entities.get_all_entities();

    let skip_list = Vec::<(i32, i32)>::new();
    todo!("Iterate over matrix!");
    for (pos, entity) in entities {
        let neighbours = positioned_entities.get_neighbours_of_position(*pos);
        let neighbour_count = neighbours.len();
        if neighbour_count < 2 || neighbour_count > 3 {
            commands.entity(*entity).despawn();
        }
    }
}
