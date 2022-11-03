use std::time::Duration;
use console_engine::KeyCode;

use crate::inputmanager::axis::Axis::Horizontal;
use crate::inputmanager::axis::Axis::Vertical;

use crate::resources::axis_inputs::AxisInputs;
use crate::resources::key_bindings::KeyBindings;
use crate::resources::render_targets::RenderTargets;
use crate::resources::timer::Timer;
use crate::systems::reset_axis_input::reset_axial_inputs;
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

    let input_value = inputs.get(Horizontal);

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
    assert_eq!(inputs.get(Horizontal), 1.);
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

    assert_eq!(inputs.get(Horizontal), 1.);

    test_env.game.run_schedule();

    inputs = test_env.game.get_world_ref().get_resource::<AxisInputs>().unwrap();

    assert_eq!(0., inputs.get(Horizontal));
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
fn can_add_key_bindings() {
    let mut test_env = initialize();
    let keybinding_resource = KeyBindings::new();

    test_env.game.get_world_mut().insert_resource(keybinding_resource);

    let inputs = test_env.game.get_world_ref().get_resource::<KeyBindings>();
    assert!(matches!(inputs, Some(_)));
}


#[test]
fn can_add_key_to_key_bindings() {
    let mut test_env = initialize();
    let mut keybinding_resource = KeyBindings::new();

    keybinding_resource.bind_key_to_axis(
        Horizontal, 
        KeyCode::Char('d'), 
        KeyCode::Char('a')
    );

    test_env.game.get_world_mut().insert_resource(keybinding_resource);

    let inputs = test_env.game.get_world_mut().get_resource_mut::<KeyBindings>();
    assert!(matches!(inputs, Some(_)));
    let mut keybindings = inputs.unwrap();

    let horizontals = keybindings.get_axial_bindings(Horizontal);

    assert_eq!(horizontals.len(), 1);

    let verticals = keybindings.get_axial_bindings(Vertical);
    assert_eq!(verticals.len(), 0);


}
