use bevy_ecs::prelude::ResMut;

use crate::resources::inputs::mouse_inputs::MouseInputs;
use crate::resources::inputs::SharedInputBehaviour;

pub fn reset_mouse_inputs(mut input: ResMut<MouseInputs>) {
    input.reset_inputs();
}