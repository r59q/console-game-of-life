use bevy_ecs::prelude::ResMut;

use crate::input_manager::SharedInputBehaviour;
use crate::resources::inputs::mouse_inputs::MouseInputs;

pub fn reset_mouse_inputs(mut input: ResMut<MouseInputs>) {
    input.reset_inputs();
}