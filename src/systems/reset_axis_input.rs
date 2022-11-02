use bevy_ecs::prelude::ResMut;

use crate::{inputmanager::Input, resources::axis_inputs::AxisInputs};

// Todo: Make inputs a trait with reset inputs function
pub fn reset_axial_inputs(mut input: ResMut<AxisInputs>) {
    input.reset_inputs();
}