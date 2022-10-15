use std::time::Duration;

use crate::resources::timer::Timer;

use super::*;

#[test]
fn can_add_timer_resource() {
    let mut test_env = initialize();

    let time_resource = Timer::new();

    test_env.game.get_world().insert_resource(time_resource);

    let time = test_env.game.get_world().get_resource::<Timer>();

    match time {
        None => { assert!(false) }
        Some(_) => { assert!(true) }
    }
}

#[test]
fn timer_duration_changes_over_time() {
    let mut test_env = initialize();

    let time_resource = Timer::new();

    test_env.game.get_world().insert_resource(time_resource);

    let time = test_env.game.get_world().get_resource::<Timer>();

    let first_time_duration = std::time::SystemTime::now().duration_since(time.unwrap().start_time).unwrap();
    std::thread::sleep(Duration::from_micros(100));
    let second_time_duration = std::time::SystemTime::now().duration_since(time.unwrap().start_time).unwrap();
    assert_ne!(first_time_duration.as_micros(), second_time_duration.as_micros())
}

#[test]
fn timer_with_timing_system_contains_last_frame_delta() {
    let mut test_env = initialize();

    let time_resource = Timer::new();

    test_env.game.get_world().insert_resource(time_resource);

    let _time = test_env.game.get_world().get_resource::<Timer>();

    assert!(false)
}