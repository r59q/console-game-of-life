use bevy_ecs::prelude::Res;
use console_engine::KeyCode;

use crate::resources::inputs::{Inputs, MouseButton};

pub fn debug_inputs(input: Res<Inputs>) {
    let is_a_held = input.get_mouse_down_or_held(MouseButton::LEFT);
    match is_a_held
    {
        true => {
            let pos = input.get_mouse_position();
            if let Some(pos) = pos {
                println!("pos {:?}", pos)
            }
            println!("BTN is down!")
        }
        false => {}
    }
}