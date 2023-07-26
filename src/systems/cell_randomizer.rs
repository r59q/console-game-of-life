use bevy_ecs::system::{Commands, Res, ResMut};
use rand::random;

use crate::{
    input_manager::buttons::Button,
    resources::{inputs::button_inputs::ButtonInputs, positioned_entities::PositionedEntities},
};

use super::utils::cell_spawning::spawn_cell_at;

const RANDOM_CELL_AREA_SIZE: (i32, i32) = (100, 50);
const RANDOM_CELL_AREA_OFFSET: (i32, i32) = (20, 5);

pub fn cell_randomizer(
    button_inputs: Res<ButtonInputs>,
    positioned_entities: ResMut<PositionedEntities>,
    mut commands: Commands,
) {
    if !button_inputs.get_btn_down(Button::Random) {
        return;
    }

    let x_start = RANDOM_CELL_AREA_OFFSET.0;
    let y_start = RANDOM_CELL_AREA_OFFSET.1;

    let x_end = RANDOM_CELL_AREA_SIZE.0 + x_start;
    let y_end = RANDOM_CELL_AREA_SIZE.1 + y_start;

    let positioned_entities_inner = positioned_entities.into_inner();

    for x in x_start..x_end {
        for y in y_start..y_end {
            if random() {
                spawn_cell_at((x, y), &mut commands, positioned_entities_inner);
            }
        }
    }
}
