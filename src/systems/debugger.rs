use bevy_ecs::prelude::{Res, ResMut};
use console_engine::MouseButton;

use strum::IntoEnumIterator;

use crate::input_manager::axis::Axis;
use crate::input_manager::buttons::Button::{Buy};
use crate::resources::axis_inputs::AxisInputs;
use crate::resources::button_inputs::ButtonInputs;
use crate::resources::mouse_inputs::MouseInputs;

pub fn debugger(axial_inputs: Res<AxisInputs>, mut mouse_inputs: ResMut<MouseInputs>, button_inputs: Res<ButtonInputs>) {

    for axis in Axis::iter() {
        let val = axial_inputs.get(&axis);
        print!("{:?}", axis);
        print!("{:?}", val);
    }

    let state = mouse_inputs.get_state(MouseButton::Left);
    print!("\t {:?}", state);

    print!("\t {:?}", mouse_inputs.get_position());

    let fire1_action = button_inputs.get_btn_action(Buy);

    print!("\t {:?}", fire1_action);
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