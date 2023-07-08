use std::process::CommandArgs;

use crate::components::placeable::Placeable;
use crate::components::position::Position;
use crate::input_manager::buttons::Button;
use crate::input_manager::input_action::InputAction;
use crate::resources::inputs::button_inputs::ButtonInputs;
use crate::resources::pause_state::PauseState;
use bevy_ecs::prelude::Query;
use bevy_ecs::system::{Commands, Res};

pub fn spawn_placeables(
    query: Query<(&Position, &Placeable)>,
    button_inputs: Res<ButtonInputs>,
    mut commands: Commands,
    pause_state: Res<PauseState>,
) {
    if !pause_state.is_paused() {
        return;
    }

    if button_inputs.get_btn_action(Button::Place).clone() != InputAction::Down {
        return;
    }
    for (pos, placeable) in &query {
        let position = pos.clone();
        if let Some(replacement) = placeable.replacement {
            let replacement_entity = replacement(commands.spawn());

            commands
                .entity(replacement_entity)
                .remove::<Position>()
                .insert(position);
        }
    }
}
