use bevy_ecs::prelude::{Query, ResMut};
use bevy_ecs::system::{Commands, Res};
use bevy_ecs::world::EntityMut;
use crate::components::placeable::Placeable;
use crate::components::position::Position;
use crate::input_manager::buttons::Button;
use crate::input_manager::input_action::InputAction;
use crate::resources::inputs::button_inputs::ButtonInputs;

pub fn spawn_placeables(query: Query<(&Position, &Placeable)>, button_inputs: Res<ButtonInputs>, mut commands: Commands) {
    if button_inputs.get_btn_action(Button::Place).clone() != InputAction::Down {
        return;
    }
    for (pos, placeable) in &query {
        let position = pos.clone();
        if let Some(replacement) = placeable.replacement {
            let replacement_entity = replacement(commands.spawn());
            commands.entity(replacement_entity).remove::<Position>().insert(position);
        }
    }
}