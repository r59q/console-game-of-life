use bevy_ecs::schedule::SystemStage;

use crate::{tests::initialize_with_entity_and_timing_system, resources::{inputs::axis_inputs::AxisInputs, pause_state::PauseState}, components::{position::Position, velocity::Velocity, controllable::Controllable}, systems::axis_position_transform::axis_position_transform, input_manager::axis::Axis};



#[test]
fn can_add_axis_position_transform_system() {
    let mut test_env = initialize_with_entity_and_timing_system();

    test_env.game.get_world_mut().insert_resource(AxisInputs::new());
    test_env.game.get_world_mut().insert_resource(PauseState::new());

    // Create some entities
    let entity1 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. })
        .insert(Controllable { }).id();

    // Run the game
    test_env.game.add_stage_to_schedule(
        "preupdate",
        SystemStage::parallel()
            .with_system(axis_position_transform)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Axis::Horizontal,1.);
    axis_inputs.set(Axis::Vertical,1.);

    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    assert_ne!(pos1.x, 0.);
    assert_ne!(pos1.y, 0.);
}

#[test]
fn cannot_move_with_axis_transform_system_when_paused() {
    let mut test_env = initialize_with_entity_and_timing_system();

    test_env.game.get_world_mut().insert_resource(AxisInputs::new());
    let mut pause_state = PauseState::new();
    pause_state.pause();
    test_env.game.get_world_mut().insert_resource(pause_state);

    // Create some entities
    let entity1 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. })
        .insert(Controllable { }).id();

    // Run the game
    test_env.game.add_stage_to_schedule(
        "preupdate",
        SystemStage::parallel()
            .with_system(axis_position_transform)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Axis::Horizontal,1.);
    axis_inputs.set(Axis::Vertical,1.);

    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    assert_eq!(pos1.x, 0.);
    assert_eq!(pos1.y, 0.);
}

#[test]
fn axis_position_transform_system_does_not_work_without_controllable_component() {
    let mut test_env = initialize_with_entity_and_timing_system();

    test_env.game.get_world_mut().insert_resource(AxisInputs::new());
    test_env.game.get_world_mut().insert_resource(PauseState::new());

    // Create some entities
    let entity1 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. }).id();

    // Run the game
    test_env.game.add_stage_to_schedule(
        "preupdate",
        SystemStage::parallel()
            .with_system(axis_position_transform)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Axis::Horizontal,1.);
    axis_inputs.set(Axis::Vertical,1.);

    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    assert_eq!(pos1.x, 0.);
    assert_eq!(pos1.y, 0.);
}

#[test]
fn moves_multiple_times_with_transform_system() {
    let mut test_env = initialize_with_entity_and_timing_system();

    test_env.game.get_world_mut().insert_resource(AxisInputs::new());
    test_env.game.get_world_mut().insert_resource(PauseState::new());

    // Create some entities
    let entity1 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. })
        .insert(Controllable { }).id();

    // Run the game
    test_env.game.add_stage_to_schedule(
        "preupdate",
        SystemStage::parallel()
            .with_system(axis_position_transform)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Axis::Horizontal,1.);
    axis_inputs.set(Axis::Vertical,1.);

    test_env.game.run_schedule();
    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    assert_eq!(pos1.x, 2.);
    assert_eq!(pos1.y, 2.);
}

