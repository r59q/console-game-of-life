use crate::components::position::Position;
use crate::input_manager::buttons::Button;
use crate::prefabs::Prefabs;
use crate::resources::positioned_entities::{self, PositionedEntities};
use crate::{
    input_manager::input_action::InputAction,
    resources::{
        inputs::{button_inputs::ButtonInputs, mouse_inputs::MouseInputs},
        view_offset::ViewOffset,
    },
};
use bevy_ecs::system::{Commands, Res, ResMut};

pub fn toggle_cell_on_click(
    mouse_inputs: Res<MouseInputs>,
    view_offset: Res<ViewOffset>,
    button_inputs: Res<ButtonInputs>,
    mut positioned_entities: ResMut<PositionedEntities>,
    mut commands: Commands,
) {
    let should_spawn_cell =
        button_inputs.get_btn_action(Button::Fire1).clone() == InputAction::Down;

    if should_spawn_cell {
        let target_pos = view_offset.get_mouse_to_world_offset_i32(mouse_inputs.get_position());

        let new_cell_pos_component = Position {
            x: target_pos.0 as f64,
            y: target_pos.1 as f64,
        };
        let placed_entity =
            Prefabs::CELL_WITHOUT_POSITION(&mut commands.spawn().insert(new_cell_pos_component));
        positioned_entities.put_entity(placed_entity, new_cell_pos_component);
    }
}
