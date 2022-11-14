use bevy_ecs::system::{ResMut, Res};

use crate::resources::{view_offset::ViewOffset, inputs::mouse_inputs::MouseInputs};

pub fn drag_view_offset(mut offset: ResMut<ViewOffset>, mouse_inputs: Res<MouseInputs>) {
    let (delta_x, delta_y) = mouse_inputs.get_drag_delta();
    let norm_delta_x: i64 = delta_x.into();
    let norm_delta_y: i64 = delta_y.into();

    let (offset_x, offset_y) = offset.get_offset();
    offset.set_offset(offset_x - norm_delta_x, offset_y - norm_delta_y);
}