use bevy_ecs::prelude::ResMut;

use crate::resources::inputs::axis_inputs::AxisInputs;
use crate::resources::inputs::SharedInputBehaviour;

pub fn reset_axial_inputs(mut input: ResMut<AxisInputs>) {
    input.reset_inputs();
}