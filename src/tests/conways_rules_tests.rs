use std::{
    thread::Thread,
    time::{Duration, SystemTime},
};

use bevy_ecs::schedule::SystemStage;

use crate::{
    components::position::Position,
    resources::{
        pause_state::PauseState, positioned_entities::PositionedEntities,
        simulation_speed::SimulationSpeed, timer::Timer,
    },
    systems::{
        conways_rules::{self, conways_rules},
        positioned_entities_updater::positioned_entities_updater,
    },
    tests::initialize,
};

use super::TestEnv;

#[test]
fn cell_dies_if_no_neighbours_are_present() {
    let mut test_env = initialize_with_conway();

    test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 0., y: 0. });

    std::thread::sleep(Duration::from_millis(15));
    test_env.game.run_schedule();
    test_env.game.run_schedule();

    let positioned_entities = test_env
        .game
        .get_world_ref()
        .get_resource::<PositionedEntities>()
        .unwrap();

    assert!(positioned_entities.get(&(0, 0)).is_none());
}

#[test]
fn cell_dies_if_one_neighbour_are_present() {
    let mut test_env = initialize_with_conway();

    test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 0., y: 0. });
    test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 1., y: 1. });

    std::thread::sleep(Duration::from_millis(15));
    test_env.game.run_schedule();
    test_env.game.run_schedule();

    let positioned_entities = test_env
        .game
        .get_world_ref()
        .get_resource::<PositionedEntities>()
        .unwrap();

    assert!(positioned_entities.get(&(0, 0)).is_none());
}
#[test]
fn does_not_run_when_game_is_paused() {
    let mut test_env = initialize_with_conway();

    test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 0., y: 0. });

    test_env
        .game
        .get_world_mut()
        .get_resource_mut::<PauseState>()
        .unwrap()
        .pause();

    std::thread::sleep(Duration::from_millis(15));
    test_env.game.run_schedule();
    test_env.game.run_schedule();

    let positioned_entities = test_env
        .game
        .get_world_ref()
        .get_resource::<PositionedEntities>()
        .unwrap();

    assert!(positioned_entities.get(&(0, 0)).is_some());
}

#[test]
fn cell_dies_if_greater_than_3_neighbours_are_present() {
    let mut test_env = initialize_with_conway();

    test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 0., y: 0. });
    test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 1., y: 1. });
    test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 1., y: -1. });
    test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: -1., y: 1. });
    test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: -1., y: -1. });

    std::thread::sleep(Duration::from_millis(15));
    test_env.game.run_schedule();
    test_env.game.run_schedule();

    let positioned_entities = test_env
        .game
        .get_world_ref()
        .get_resource::<PositionedEntities>()
        .unwrap();

    assert!(positioned_entities.get(&(0, 0)).is_none());
}

#[test]
fn cell_should_spawn_if_exactly_three_neighbours() {
    let mut test_env = initialize_with_conway();
    test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 0., y: 0. });

    test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 0., y: 1. });

    test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 1., y: 0. });

    std::thread::sleep(Duration::from_millis(15));
    test_env.game.run_schedule();
    test_env.game.run_schedule();

    let positioned_entities = test_env
        .game
        .get_world_ref()
        .get_resource::<PositionedEntities>()
        .unwrap();

    assert!(positioned_entities.get(&(1, 1)).is_some());
}

#[test]
fn conway_rules_reset_cell_update_timer() {
    let mut test_env = initialize_with_conway();
    std::thread::sleep(Duration::from_millis(15));
    test_env.game.run_schedule();

    let timer = test_env
        .game
        .get_world_ref()
        .get_resource::<Timer>()
        .unwrap();

    assert_eq!(0, timer.get_time_since_cell_updates().as_millis());
}

#[test]
fn conway_rules_doesnt_reset_if_duration_too_short() {
    let mut test_env = initialize_with_conway();

    std::thread::sleep(Duration::from_millis(5));
    test_env.game.run_schedule();

    let timer = test_env
        .game
        .get_world_ref()
        .get_resource::<Timer>()
        .unwrap();

    assert_ne!(0, timer.get_time_since_cell_updates().as_millis());
}

fn initialize_with_conway() -> TestEnv {
    let mut test_env = initialize();
    test_env
        .game
        .get_world_mut()
        .insert_resource(PauseState::new());
    test_env
        .game
        .get_world_mut()
        .insert_resource(PositionedEntities::new());
    test_env.game.get_world_mut().insert_resource(Timer::new());
    test_env
        .game
        .get_world_mut()
        .insert_resource(SimulationSpeed::new(10));

    test_env.game.add_stage_to_schedule(
        "stage1",
        SystemStage::parallel().with_system(positioned_entities_updater),
    );

    test_env
        .game
        .add_stage_to_schedule("stage2", SystemStage::parallel().with_system(conways_rules));
    test_env
}
