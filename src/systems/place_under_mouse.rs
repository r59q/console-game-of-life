use bevy_ecs::query::With;
use bevy_ecs::system::{Query, Res};
use crate::components::placeable::Placeable;
use crate::components::position::Position;

use crate::resources::inputs::mouse_inputs::MouseInputs;
use crate::resources::view_offset::ViewOffset;


pub fn place_under_mouse(mut query: Query<&mut Position, With<Placeable>>, mouse_inputs :Res<MouseInputs>, view_offset :Res<ViewOffset>) {
    let m_pos_x:i32 = mouse_inputs.get_position().0 as i32;
    let m_pos_y:i32 = mouse_inputs.get_position().1 as i32;

    let view_offset_x:i32 = view_offset.get_offset().0 as i32;
    let view_offset_y:i32 = view_offset.get_offset().1 as i32;

    let pos_x: f64 = (m_pos_x + view_offset_x).into();
    let pos_y: f64 = (m_pos_y + view_offset_y).into();

    let new_pos = Position{
        x: pos_x.into(),
        y: pos_y.into(),
    };

    for mut pos in &mut query {
        pos.x = new_pos.x;
        pos.y = new_pos.y;
    }
}