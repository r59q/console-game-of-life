use bevy_ecs::prelude::{Res, ResMut};
use console_engine_TC_FIX::MouseButton;

use strum::IntoEnumIterator;

use crate::input_manager::axis::Axis;
use crate::input_manager::buttons::Button::Buy;
use crate::resources::inputs::axis_inputs::AxisInputs;
use crate::resources::inputs::button_inputs::ButtonInputs;
use crate::resources::inputs::mouse_inputs::MouseInputs;
use crate::resources::pause_state::PauseState;
use crate::resources::positioned_entities::PositionedEntities;
use crate::resources::view_offset::ViewOffset;

pub fn debugger(
    axial_inputs: Res<AxisInputs>,
    mouse_inputs: ResMut<MouseInputs>,
    button_inputs: Res<ButtonInputs>,
    pause_state: Res<PauseState>,
    view_offset: Res<ViewOffset>,
    positioned_entities: Res<PositionedEntities>,
) {
    for axis in Axis::iter() {
        let val = axial_inputs.get(&axis);
        print!("{:?}", axis);
        print!("{:?}", val);
    }

    let state = mouse_inputs.get_state(MouseButton::Left);
    print!("\t {:?}", state);

    let mouse_position = mouse_inputs.get_position();
    print!("\t {:?}", mouse_position);

    let fire1_action = button_inputs.get_btn_action(Buy);

    print!("\t {:?}", fire1_action);

    print!("\t Paused:{:?}", pause_state.is_paused());

    print!("\t {:?}", fire1_action);

    let world_pos = view_offset.get_mouse_to_world_offset_i32(mouse_position);
    let neighbours_of_mouse_pos = positioned_entities
        .get_neighbours_of_position(world_pos)
        .len();

    print!("\t {:?}", neighbours_of_mouse_pos);
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
