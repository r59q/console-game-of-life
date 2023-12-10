use bevy_ecs::system::{Res, ResMut};

use crate::{
    input_manager::input_action::InputAction,
    resources::{inputs::mouse_inputs::MouseInputs, view_offset::ViewOffset},
};

pub fn drag_view_offset(mut offset: ResMut<ViewOffset>, mouse_inputs: Res<MouseInputs>) {
    if mouse_inputs
        .get_state(console_engine_TC_FIX::MouseButton::Left)
        .get_action()
        == InputAction::Drag
    {
        return;
    }

    let (delta_x, delta_y) = mouse_inputs.get_drag_delta();
    let norm_delta_x: i64 = delta_x.into();
    let norm_delta_y: i64 = delta_y.into();

    let (offset_x, offset_y) = offset.get_offset();
    offset.set_offset(offset_x - norm_delta_x, offset_y - norm_delta_y);
}
