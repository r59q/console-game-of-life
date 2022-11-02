use std::time::Duration;

use crate::resources::axis_inputs::AxisInputs;
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

// #[test]
// fn can_read_from_input_resource() {
//     let mut test_env = initialize();

//     let input_resource = Inputs::new();

//     test_env.game.get_world_mut().insert_resource(input_resource);

//     let inputs = test_env.game.get_world_ref().get_resource::<Inputs>().unwrap();

//     let key_down = inputs.get_key_down(KeyCode::Char('t'));

//     assert_eq!(false, key_down)
// }

// #[test]
// fn can_set_input_resource() {
//     let mut test_env = initialize();
//     let input_resource = Inputs::new();
//     test_env.game.get_world_mut().insert_resource(input_resource);

//     let mut inputs_mut = test_env.game.get_world_mut().get_resource_mut::<Inputs>().unwrap();
//     inputs_mut.register_key_press(KeyCode::Char('k'), Direction::DOWN);

//     let inputs = test_env.game.get_world_ref().get_resource::<Inputs>().unwrap();
//     let key_down = inputs.get_key_down(KeyCode::Char('k'));

//     assert_eq!(true, key_down);
// }

// #[test]
// fn system_can_reset_inputs() {
//     let mut test_env = initialize();
//     let input_resource = Inputs::new();
//     test_env.game.get_world_mut().insert_resource(input_resource);

//     test_env.game.add_stage_to_schedule(
//         "test",
//         SystemStage::parallel().with_system(reset_inputs),
//     );

//     let mut inputs_mut = test_env.game.get_world_mut().get_resource_mut::<Inputs>().unwrap();
//     inputs_mut.register_key_press(KeyCode::Char('k'), Direction::DOWN);

//     let mut inputs = test_env.game.get_world_ref().get_resource::<Inputs>().unwrap();
//     let mut key_down = inputs.get_key_down(KeyCode::Char('k'));

//     assert_eq!(true, key_down);

//     test_env.game.run_schedule();

//     inputs = test_env.game.get_world_ref().get_resource::<Inputs>().unwrap();
//     key_down = inputs.get_key_down(KeyCode::Char('k'));

//     assert_eq!(false, key_down);
// }

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
