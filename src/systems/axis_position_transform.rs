use bevy_ecs::query::With;
use bevy_ecs::system::{Query, Res};
use crate::components::controllable::Controllable;
use crate::components::position::Position;
use crate::input_manager::axis::Axis::{Horizontal, Vertical};
use crate::resources::inputs::axis_inputs::AxisInputs;
use crate::resources::pause_state::PauseState;

pub fn axis_position_transform(mut query: Query<&mut Position, With<Controllable>>, axis_inputs :Res<AxisInputs>, pause_state: Res<PauseState>) {
    if pause_state.is_paused() {
        return;
    }
    for mut position in &mut query {
        position.x += axis_inputs.get(&Horizontal);
        position.y += axis_inputs.get(&Vertical);
    }
}