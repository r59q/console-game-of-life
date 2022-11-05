use console_engine::{ConsoleEngine, MouseButton};
use crate::{game::Game, resources::mouse_inputs::{MouseInputs}};
use crate::input_manager::buttons::Button;
use crate::input_manager::input_action::InputAction;
use crate::resources::bindings::Bindings;
use strum::IntoEnumIterator;
use crate::input_manager::axis::Axis;
use crate::input_manager::input_action::InputAction::{Down, Held, Up};
use crate::input_manager::input_types::InputType;
use crate::resources::axis_inputs::AxisInputs;
use crate::resources::button_inputs::ButtonInputs;

fn handle_input(input: InputType, engine: &ConsoleEngine) -> InputAction {
    return match input {
        InputType::Key(key_code) => {
            if engine.is_key_pressed(key_code) {
                return Down
            }
            if engine.is_key_held(key_code) {
                return Held
            }
            if engine.is_key_released(key_code) {
                return Up
            }
            InputAction::None
        }
        InputType::Mouse(mouse_button) => {
            if let Some((_, _)) = engine.get_mouse_press(mouse_button) {
                return Down
            }
            if let Some((_, _)) = engine.get_mouse_released(mouse_button) {
                return Up
            }
            if let Some((_, _)) = engine.get_mouse_held(mouse_button) {
                return Held
            }
            InputAction::None
        }
    }
}

pub fn capture_axial_inputs(game: &mut Game) {
    for axis in Axis::iter() {
        let axis_binding_opt
            = game.get_world_ref().get_resource::<Bindings>().unwrap()
            .get_axial_bindings(axis);

        if let Some(axial_binding) = axis_binding_opt {
            let axial_binding= axial_binding.clone();
            for binding in axial_binding {
                let engine = game.get_engine();
                let action_positive = handle_input(binding.positive, engine);
                let action_negative = handle_input(binding.negative, engine);
                let pos_val = match action_positive {
                    Down => 1.,
                    Held => 1.,
                    _ => 0.
                };
                let neg_val = match action_negative {
                    Down => 1.,
                    Held => 1.,
                    _ => 0.
                };
                game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap()
                    .set(axis, -neg_val + pos_val);

            }
        }
    }
}
pub fn capture_button_inputs(game: &mut Game) {

    for button in Button::iter() {
        let button_binding_opt
            = game.get_world_ref().get_resource::<Bindings>().unwrap()
            .get_button_bindings(button);
        if let Some(button_binding) = button_binding_opt {
            let button_binding= button_binding.clone();
            for binding in button_binding {
                let engine = game.get_engine();
                let action = handle_input(binding.button_input, engine);
                game.get_world_mut().get_resource_mut::<ButtonInputs>().unwrap().set_btn(button, action);
            }
        }
    }
}
pub fn capture_mouse_inputs(game: &mut Game) {
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

    let mut mouse_inputs = game.get_world_mut().get_resource_mut::<MouseInputs>().unwrap();

    // Left button inputs
    if let Some((x, y)) = m_left_press {
        mouse_inputs.set_state(MouseButton::Left, InputAction::Down, x, y);
    }
    if let Some((x, y)) = m_left_held {
        mouse_inputs.set_state(MouseButton::Left, InputAction::Drag, x, y);
    }
    if let Some((x, y)) = m_left_released {
        mouse_inputs.set_state(MouseButton::Left, InputAction::Up, x, y);
    }

    // Right button inputs
    if let Some((x, y)) = m_right_press {
        mouse_inputs.set_state(MouseButton::Right, InputAction::Down, x, y);
    }
    if let Some((x, y)) = m_right_held {
        mouse_inputs.set_state(MouseButton::Right, InputAction::Drag, x, y);
    }
    if let Some((x, y)) = m_right_released {
        mouse_inputs.set_state(MouseButton::Right, InputAction::Up, x, y);
    }

    // Middle button inputs
    if let Some((x, y)) = m_middle_press {
        mouse_inputs.set_state(MouseButton::Middle, InputAction::Down, x, y);
    }
    if let Some((x, y)) = m_middle_held {
        mouse_inputs.set_state(MouseButton::Middle, InputAction::Drag, x, y);
    }
    if let Some((x, y)) = m_middle_released {
        mouse_inputs.set_state(MouseButton::Middle, InputAction::Up, x, y);
    }
}

/*for button in Button::iter() {

        let button_binding_opt
            = game.get_world_ref().get_resource::<Bindings>().unwrap()
            .get_button_bindings(button);
        if let None = button_binding_opt {
            panic!("No button binding available")
        }

        let button_inputs_opt = game.get_world_mut().get_resource_mut::<ButtonInputs>();
        if let None = button_inputs_opt {
            panic!("No button inputs available")
        }

        for binding in game.get_world_ref().get_resource::<Bindings>().unwrap()
            .get_button_bindings(button).unwrap() {
            let input_type = binding.button_input;
            match input_type {
                InputType::Key(key_code) => {
                    if game.get_engine().is_key_pressed(key_code) {
                        let mut button_inputs = game.get_world_mut().get_resource_mut::<ButtonInputs>().unwrap();
                        button_inputs.set_btn(button, Down)
                    }
                    if game.get_engine().is_key_held(key_code) {
                        let mut button_inputs = game.get_world_mut().get_resource_mut::<ButtonInputs>().unwrap();
                        button_inputs.set_btn(button, Held)
                    }
                    if game.get_engine().is_key_released(key_code) {
                        let mut button_inputs = game.get_world_mut().get_resource_mut::<ButtonInputs>().unwrap();
                        button_inputs.set_btn(button, Up)
                    }
                }
                InputType::Mouse(mouse_button) => {
                    if let Some((x,y)) = game.get_engine().get_mouse_held(mouse_button) {
                        let mut button_inputs = game.get_world_mut().get_resource_mut::<ButtonInputs>().unwrap();
                        button_inputs.set_btn(button, Down)
                    }
                    if let Some((x,y)) = game.get_engine().get_mouse_released(mouse_button) {
                        let mut button_inputs = game.get_world_mut().get_resource_mut::<ButtonInputs>().unwrap();
                        button_inputs.set_btn(button, Up)
                    }
                    if let Some((x,y)) = game.get_engine().get_mouse_held(mouse_button) {
                        let mut button_inputs = game.get_world_mut().get_resource_mut::<ButtonInputs>().unwrap();
                        button_inputs.set_btn(button, Held)
                    }
                }
            }
        }
    }*/