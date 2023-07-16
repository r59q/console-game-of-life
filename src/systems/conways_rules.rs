use bevy_ecs::system::{Commands, Res};

use crate::{
    components::position::Position,
    prefabs::Prefabs,
    resources::{
        pause_state::{self, PauseState},
        positioned_entities::PositionedEntities,
    },
};

fn get_neighbour_matrix((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    let mut matrix = Vec::<(i32, i32)>::new();
    for x_offset in -1..2 {
        for y_offset in -1..2 {
            if x_offset != 0 || y_offset != 0 {
                matrix.push((x + x_offset, y + y_offset))
            }
        }
    }
    matrix
}

pub fn conways_rules(
    mut commands: Commands,
    positioned_entities: Res<PositionedEntities>,
    pause_state: Res<PauseState>,
) {
    if pause_state.is_paused() {
        return;
    }

    // These are all cells currently activated
    let entities = positioned_entities.get_all_entities();

    let mut skip_list = Vec::<(i32, i32)>::new();

    for (pos, entity) in &entities {
        let neighbours = positioned_entities.get_neighbours_of_position(**pos);
        let neighbour_count = neighbours.len();
        if neighbour_count < 2 || neighbour_count > 3 {
            commands.entity(**entity).despawn();
        }
        skip_list.push(**pos);
    }
    for (pos, _entity) in &entities {
        let neighbour_matrix = get_neighbour_matrix(**pos);
        for matrix_pos in neighbour_matrix.into_iter() {
            if !skip_list.contains(&matrix_pos) {
                let matrix_pos_neighbours =
                    positioned_entities.get_neighbours_of_position(matrix_pos);
                if matrix_pos_neighbours.len() == 3 {
                    // Spawn cell here
                    Prefabs::CELL_WITHOUT_POSITION(commands.spawn().insert(Position {
                        x: f64::from(matrix_pos.0),
                        y: f64::from(matrix_pos.1),
                    }));
                }
                skip_list.push(matrix_pos);
            }
        }
    }
}
