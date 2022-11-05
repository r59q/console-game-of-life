use bevy_ecs::change_detection::Mut;
use bevy_ecs::schedule::ShouldRun::No;
use console_engine::{ConsoleEngine, MouseButton};
use crate::{game::Game, resources::mouse_inputs::{MouseInputs}};
use crate::inputmanager::buttons::Button;
use crate::inputmanager::input_action::InputAction;
use crate::resources::bindings::Bindings;
use strum::IntoEnumIterator;
use crate::inputmanager::button_binding::ButtonBinding;
use crate::inputmanager::input_action::InputAction::{Down, Drag, Held, Up};
use crate::inputmanager::input_types::InputType;
use crate::resources::button_inputs::ButtonInputs;


/*pub fn capture_inputs(game: &mut Game) {
    let mouse_inputs_opt = game.get_world_mut().get_resource_mut::<MouseInputs>();
    let engine = game.get_engine();
    capture_mouse_inputs(engine, mouse_inputs_opt.unwrap());

    let bindings_opt = game.get_world_mut().get_resource_mut::<Bindings>();
    if let None = bindings_opt {
        panic!("NO BINDINGS ARE SET!")
    }

    // capture_button_inputs(game, bindings_opt.unwrap())
}

*/

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
            if let Some((x, y)) = engine.get_mouse_press(mouse_button) {
                return Down
            }
            if let Some((x, y)) = engine.get_mouse_released(mouse_button) {
                return Up
            }
            if let Some((x, y)) = engine.get_mouse_held(mouse_button) {
                return Held
            }
            InputAction::None
        }
    }
}

pub fn capture_button_inputs(game: &mut Game) {
    let bindings_opt = game.get_world_ref().get_resource::<Bindings>();
    if let None = bindings_opt {
        panic!("There are no bindings")
    }

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