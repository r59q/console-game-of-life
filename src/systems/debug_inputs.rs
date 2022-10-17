use bevy_ecs::prelude::Res;
use console_engine::KeyCode;

use crate::resources::inputs::{Inputs, MouseButton};

pub fn debug_inputs(input: Res<Inputs>) {
    let a = input.get_key_down_or_held(KeyCode::Char('a'));
    let d = input.get_key_down_or_held(KeyCode::Char('d'));
    let s = input.get_key_down_or_held(KeyCode::Char('s'));
    let w = input.get_key_down_or_held(KeyCode::Char('w'));

    if a {
        println!("a")
    }
    if s {
        println!("s")
    }
    if d {
        println!("d")
    }
    if w {
        println!("w")
    }
}