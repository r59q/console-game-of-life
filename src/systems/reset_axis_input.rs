use bevy_ecs::prelude::ResMut;

use crate::input_manager::SharedInputBehaviour;
use crate::resources::inputs::axis_inputs::AxisInputs;

pub fn reset_axial_inputs(mut input: ResMut<AxisInputs>) {
    input.reset_inputs();
}