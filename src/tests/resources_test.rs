use std::time::Duration;
use console_engine::KeyCode;
use console_engine::MouseButton;

use crate::inputmanager::axis::Axis::Horizontal;
use crate::inputmanager::axis::Axis::Vertical;
use crate::inputmanager::button_binding::ButtonBinding;
use crate::inputmanager::input_action::InputAction;
use crate::inputmanager::input_types::InputType::Key;

use crate::resources::axis_inputs::AxisInputs;
use crate::resources::bindings::Bindings;
use crate::resources::button_inputs::ButtonInputs;
use crate::resources::mouse_inputs::MouseInputs;
use crate::resources::render_targets::RenderTargets;
use crate::resources::timer::Timer;
use crate::systems::reset_axis_input::reset_axial_inputs;
use crate::systems::reset_mouse_input;
use crate::systems::reset_mouse_input::reset_mouse_inputs;
use crate::systems::timing::timing_system;

use super::*;

#[test]
fn can_add_timer_resource() {
    let mut test_env = initialize();

    let time_resource = Timer::new();

    test_env.game.get_world_mut().insert_resource(time_resource);

    let time = test_env.game.get_world_mut().get_resource::<Timer>();

    match time {
        None => { assert!(false) }
        Some(_) => { assert!(true) }
    }
}

#[test]
fn timer_duration_changes_over_time() {
    let mut test_env = initialize();

    let time_resource = Timer::new();

    test_env.game.get_world_mut().insert_resource(time_resource);

    let time = test_env.game.get_world_mut().get_resource::<Timer>();

    let first_time_duration = std::time::SystemTime::now().duration_since(time.unwrap().start_time).unwrap();
    std::thread::sleep(Duration::from_micros(100));
    let second_time_duration = std::time::SystemTime::now().duration_since(time.unwrap().start_time).unwrap();
    assert_ne!(first_time_duration.as_micros(), second_time_duration.as_micros())
}

#[test]
fn timer_with_timing_system_contains_last_frame_delta() {
    let test_env = initialize();
    let mut game = test_env.game;
    let time_resource = Timer::new();
    game.get_world_mut().insert_resource(time_resource);
    let timing_system = timing_system;
    let mut schedule = Schedule::default();

    schedule.add_stage("timer", SystemStage::parallel().with_system(timing_system));

    let world_mut = game.get_world_mut();

    let mut time = world_mut.get_resource::<Timer>();

    let first_frame_delta = time.unwrap().delta_time.as_micros();

    schedule.run(world_mut);
    schedule.run(world_mut);

    time = world_mut.get_resource::<Timer>();

    let second_frame_delta = time.unwrap().delta_time.as_micros();

    schedule.run(world_mut);

    time = world_mut.get_resource::<Timer>();

    let third_frame_delta = time.unwrap().delta_time.as_micros();

    assert_eq!(first_frame_delta, 0);
    assert_ne!(second_frame_delta, 0);
    assert_ne!(second_frame_delta, third_frame_delta);
}

#[test]
fn can_add_rendering_target() {
    let mut test_env = initialize();

    let render_targets_resource = RenderTargets::new();

    test_env.game.get_world_mut().insert_resource(render_targets_resource);

    let render_targets = test_env.game.get_world_ref().get_resource::<RenderTargets>();

    match render_targets {
        None => { assert!(false) }
        Some(_) => { assert!(true) }
    }
}


#[test]
fn can_add_input_resource() {
    let mut test_env = initialize();

    let input_resource = AxisInputs::new();

    test_env.game.get_world_mut().insert_resource(input_resource);

    let inputs = test_env.game.get_world_ref().get_resource::<AxisInputs>();

    assert!(matches!(inputs, Some(_)));
}

#[test]
fn can_read_from_input_resource() {
    let mut test_env = initialize();

    let input_resource = AxisInputs::new();

    test_env.game.get_world_mut().insert_resource(input_resource);

    let inputs = test_env.game.get_world_ref().get_resource::<AxisInputs>().unwrap();

    let input_value = inputs.get(&Horizontal);

    assert_eq!(0., input_value)
}

#[test]
fn can_set_input_resource() {
    let mut test_env = initialize();
    let input_resource = AxisInputs::new();
    test_env.game.get_world_mut().insert_resource(input_resource);

    let mut inputs_mut = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();
    inputs_mut.set(Horizontal, 1.);

    let inputs = test_env.game.get_world_ref().get_resource::<AxisInputs>().unwrap();
    assert_eq!(inputs.get(&Horizontal), 1.);
}

#[test]
fn system_can_reset_axis_inputs() {
    let mut test_env = initialize();
    let input_resource = AxisInputs::new();
    test_env.game.get_world_mut().insert_resource(input_resource);

    test_env.game.add_stage_to_schedule(
        "test",
        SystemStage::parallel().with_system(reset_axial_inputs),
    );

    let mut inputs_mut = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();
    inputs_mut.set(Horizontal, 1.);

    let mut inputs = test_env.game.get_world_ref().get_resource::<AxisInputs>().unwrap();

    assert_eq!(inputs.get(&Horizontal), 1.);

    test_env.game.run_schedule();

    inputs = test_env.game.get_world_ref().get_resource::<AxisInputs>().unwrap();

    assert_eq!(0., inputs.get(&Horizontal));
}

#[test]
fn can_add_axis_inputs() {
    let mut test_env = initialize();
    let input_resource = AxisInputs::new();

    test_env.game.get_world_mut().insert_resource(input_resource);

    test_env.game.add_stage_to_schedule(
        "test",
        SystemStage::parallel().with_system(reset_axial_inputs),
    );

    let inputs = test_env.game.get_world_ref().get_resource::<AxisInputs>();
    assert!(matches!(inputs, Some(_)));
}

#[test]
fn can_add_mouse_inputs() {
    let mut test_env = initialize();
    let input_resource = MouseInputs::new();

    test_env.game.get_world_mut().insert_resource(input_resource);

    let inputs = test_env.game.get_world_ref().get_resource::<MouseInputs>();
    assert!(matches!(inputs, Some(_)));
}

#[test]
fn system_can_reset_mouse_inputs() {
    let mut test_env = initialize();
    let input_resource = MouseInputs::new();

    test_env.game.get_world_mut().insert_resource(input_resource);

    test_env.game.add_stage_to_schedule(
        "test",
        SystemStage::parallel().with_system(reset_mouse_inputs),
    );

    let mut inputs = test_env.game.get_world_mut().get_resource_mut::<MouseInputs>().unwrap();

    inputs.set_state(MouseButton::Left, InputAction::Down, 1, 2);

    test_env.game.run_schedule();

    let mut hopefully_reset_state = test_env.game.get_world_mut().get_resource_mut::<MouseInputs>().unwrap();
    
    assert_eq!(hopefully_reset_state.get_position(), (1, 2));
    let state = hopefully_reset_state.get_state(MouseButton::Left);
    assert_eq!(state.get_action(), InputAction::None);
}

#[test]
fn can_add_key_bindings() {
    let mut test_env = initialize();
    let keybinding_resource = Bindings::new();

    test_env.game.get_world_mut().insert_resource(keybinding_resource);

    let inputs = test_env.game.get_world_ref().get_resource::<Bindings>();
    assert!(matches!(inputs, Some(_)));
}


#[test]
fn can_add_key_to_key_bindings() {
    let mut test_env = initialize();
    let mut keybinding_resource = Bindings::new();

    keybinding_resource.bind_to_axis(
        Horizontal, 
        Key(KeyCode::Char('d')),
        Key(KeyCode::Char('a'))
    );

    test_env.game.get_world_mut().insert_resource(keybinding_resource);

    let inputs = test_env.game.get_world_mut().get_resource_mut::<Bindings>();
    assert!(matches!(inputs, Some(_)));
    let mut keybindings = inputs.unwrap();

    let horizontals = keybindings.get_axial_bindings(Horizontal);

    assert_eq!(horizontals.unwrap().len(), 1);

    let verticals = keybindings.get_axial_bindings(Vertical);
    assert!(matches!(verticals, None));
}


#[test]
fn can_add_button_inputs_resource() {
    let mut test_env = initialize();
    let mut button_bindings = ButtonInputs::new();
    test_env.game.get_world_mut().insert_resource(button_bindings);

    let inputs = test_env.game.get_world_mut().get_resource_mut::<ButtonInputs>();
    assert!(matches!(inputs, Some(_)));
}
