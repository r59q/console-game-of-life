use console_engine::MouseButton;
use crate::{game::Game, resources::mouse_inputs::{MouseInputs}};
use crate::resources::mouse_inputs::MouseAction;

pub fn capture_inputs(game: &mut Game) {
    let engine = game.get_engine();
    let m_left_press = engine.get_mouse_press(MouseButton::Left);
    let m_left_held = engine.get_mouse_held(MouseButton::Left);
    let m_left_released = engine.get_mouse_released(MouseButton::Left);

    let m_right_press = engine.get_mouse_press(MouseButton::Right);
    let m_right_held = engine.get_mouse_held(MouseButton::Right);
    let m_right_released = engine.get_mouse_released(MouseButton::Right);

    let m_middle_press = engine.get_mouse_press(MouseButton::Middle);
    let m_middle_held = engine.get_mouse_held(MouseButton::Middle);
    let m_middle_released = engine.get_mouse_released(MouseButton::Middle);

    let mouse_inputs_opt = game.get_world_mut().get_resource_mut::<MouseInputs>();
    if let None = mouse_inputs_opt {
        panic!("NO MOUSE INPUTS WERE AVAILABLE FOR CAPTURE")
    }

    let mut mouse_inputs = game.get_world_mut().get_resource_mut::<MouseInputs>().unwrap();

    // Left button inputs
    if let Some((x, y)) = m_left_press {
        mouse_inputs.set_state(MouseButton::Left, MouseAction::Down, x, y);
    }
    if let Some((x, y)) = m_left_held {
        mouse_inputs.set_state(MouseButton::Left, MouseAction::Drag, x, y);
    }
    if let Some((x, y)) = m_left_released {
        mouse_inputs.set_state(MouseButton::Left, MouseAction::Up, x, y);
    }

    // Right button inputs
    if let Some((x, y)) = m_right_press {
        mouse_inputs.set_state(MouseButton::Right, MouseAction::Down, x, y);
    }
    if let Some((x, y)) = m_right_held {
        mouse_inputs.set_state(MouseButton::Right, MouseAction::Drag, x, y);
    }
    if let Some((x, y)) = m_right_released {
        mouse_inputs.set_state(MouseButton::Right, MouseAction::Up, x, y);
    }

    // Middle button inputs
    if let Some((x, y)) = m_middle_press {
        mouse_inputs.set_state(MouseButton::Middle, MouseAction::Down, x, y);
    }
    if let Some((x, y)) = m_middle_held {
        mouse_inputs.set_state(MouseButton::Middle, MouseAction::Drag, x, y);
    }
    if let Some((x, y)) = m_middle_released {
        mouse_inputs.set_state(MouseButton::Middle, MouseAction::Up, x, y);
    }
}