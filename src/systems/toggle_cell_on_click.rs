use crate::input_manager::buttons::Button;
use crate::resources::positioned_entities::{self, PositionedEntities};
use crate::{
    input_manager::input_action::InputAction,
    resources::{
        inputs::{button_inputs::ButtonInputs, mouse_inputs::MouseInputs},
        view_offset::ViewOffset,
    },
};
use bevy_ecs::system::{Commands, Res, ResMut};

use super::utils::cell_spawning::{self, despawn_cell_at, spawn_cell_at};

pub fn toggle_cell_on_click(
    mouse_inputs: Res<MouseInputs>,
    view_offset: Res<ViewOffset>,
    button_inputs: Res<ButtonInputs>,
    positioned_entities: ResMut<PositionedEntities>,
    mut commands: Commands,
) {
    let did_press_button = button_inputs.get_btn_action(Button::Fire1).clone() == InputAction::Down;
    let target_pos = view_offset.get_mouse_to_world_offset_i32(mouse_inputs.get_position());

    let cell_already_exists = positioned_entities.get(&target_pos).is_some();

    if !did_press_button {
        return;
    }

    if cell_already_exists {
        despawn_cell_at(target_pos, &mut commands, positioned_entities.into_inner());
    } else {
        spawn_cell_at(target_pos, &mut commands, positioned_entities.into_inner());
    }
}
