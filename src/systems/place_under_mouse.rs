use bevy_ecs::query::With;
use bevy_ecs::system::{Query, Res};
use crate::components::controllable::Controllable;
use crate::components::position::Position;

use crate::resources::inputs::axis_inputs::AxisInputs;


pub fn place_under_mouse(mut query: Query<&mut Position, With<Controllable>>, axis_inputs :Res<AxisInputs>) {
}