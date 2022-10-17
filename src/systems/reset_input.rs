use bevy_ecs::prelude::ResMut;
use crate::resources::inputs::Inputs;

pub fn reset_inputs(mut input: ResMut<Inputs>) {
    input.reset_inputs();
}