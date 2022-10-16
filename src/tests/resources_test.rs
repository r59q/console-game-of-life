use std::time::Duration;
use crate::resources::render_targets::RenderTargets;

use crate::resources::timer::Timer;
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
