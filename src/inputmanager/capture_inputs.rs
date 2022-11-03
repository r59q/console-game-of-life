use crate::{game::Game, resources::mouse_inputs::{self, MouseInputs}};

pub fn capture_inputs(game: &mut Game) {
    let engine = game.get_engine();
    let m_left_held = engine.get_mouse_held(console_engine::MouseButton::Left);

    let mouse_inputs_opt = game.get_world_mut().get_resource_mut::<MouseInputs>();
    if let Some(mouse_inputs) = mouse_inputs_opt {

    } else {
        panic!("NO MOUSE INPUTS WERE AVAILABLE FOR CAPTURE")
    }

    todo!();
}