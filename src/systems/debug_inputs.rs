use bevy_ecs::prelude::Res;

use strum::IntoEnumIterator;

use crate::inputmanager::axis::Axis;
use crate::resources::axis_inputs::AxisInputs;

pub fn debug_inputs(axia_inputs: Res<AxisInputs>) {

    for axis in Axis::iter() {
        let val = axia_inputs.get(&axis);
        print!("{:?}", axis);
        print!("{:?}", val);
    }

    // let a = input.get_key_down_or_held(KeyCode::Char('a'));
    // let d = input.get_key_down_or_held(KeyCode::Char('d'));
    // let s = input.get_key_down_or_held(KeyCode::Char('s'));
    // let w = input.get_key_down_or_held(KeyCode::Char('w'));

    // if a {
    //     println!("a")
    // }
    // if s {
    //     println!("s")
    // }
    // if d {
    //     println!("d")
    // }
    // if w {
    //     println!("w")
    // }
}