use crate::components::controllable::Controllable;
use crate::components::position::Position;
use crate::components::velocity::Velocity;
use crate::resources::timer::Timer;
use crate::systems::axis_position_transform::axis_position_transform;
use crate::systems::axis_velocity::axis_velocity;
use crate::systems::movement::movement_system;
use crate::systems::timing::timing_system;

use super::*;

#[test]
fn can_add_movement_system() {
    let mut test_env = initialize();
    test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. });

    let mut schedule = Schedule::default();

    schedule.add_stage(
        "update",
        SystemStage::parallel().
            with_system(movement_system),
    );
}

#[test]
fn no_velocity_does_not_change_position() {
    let mut test_env = initialize();
    let before_x = 0.;
    let before_y = 0.;

    let velocity_x = 0.;
    let velocity_y = 0.;
    let entity = test_env.game.get_world_mut().spawn()
        .insert(Position { x: before_x, y: before_y })
        .insert(Velocity { x: velocity_x, y: velocity_y }).id();

    let mut schedule = Schedule::default();

    let timer = Timer::new();

    test_env.game.get_world_mut().insert_resource(timer);

    schedule.add_stage("timing", SystemStage::parallel().with_system(timing_system));

    schedule.add_stage(
        "update",
        SystemStage::parallel().
            with_system(movement_system),
    );

    schedule.run(test_env.game.get_world_mut());

    let pos = test_env.game.get_world_mut().entity(entity).get::<Position>().unwrap();

    assert_eq!(pos.x, before_x);
    assert_eq!(pos.y, before_y);
}

#[test]
fn some_velocity_does_change_position() {
    let mut test_env = initialize();
    let before_x = 0.;
    let before_y = 0.;

    let velocity_x = 2.;
    let velocity_y = 2.;
    let entity = test_env.game.get_world_mut().spawn()
        .insert(Position { x: before_x, y: before_y })
        .insert(Velocity { x: velocity_x, y: velocity_y }).id();

    let mut schedule = Schedule::default();

    let timer = Timer::new();

    test_env.game.get_world_mut().insert_resource(timer);

    schedule.add_stage("timing", SystemStage::parallel().with_system(timing_system));

    schedule.add_stage(
        "update",
        SystemStage::parallel().
            with_system(movement_system),
    );

    schedule.run(test_env.game.get_world_mut());

    let pos = test_env.game.get_world_mut().entity(entity).get::<Position>().unwrap();

    assert_ne!(pos.x, before_x);
    assert_ne!(pos.y, before_y);
}

#[test]
fn different_velocities_makes_different_changes() {
    let mut test_env = initialize_with_entity_and_timing_system();

    // Create some entities
    let entity1 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 1., y: 1. }).id();
    let entity2 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 2., y: 2. }).id();

    // Run the game
    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel()
            .with_system(movement_system)
    );
    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    let pos2 =  game.get_world_ref().entity(entity2).get::<Position>().unwrap();
    assert_ne!(pos1.x, 0.);
    assert_ne!(pos1.y, 0.);
    assert_ne!(pos2.x, 0.);
    assert_ne!(pos2.y, 0.);
    assert_ne!(pos1.x, pos2.x);
    assert_ne!(pos1.y, pos2.y);
}

#[test]
fn can_add_axis_velocity_system() {
    let mut test_env = initialize_with_entity_and_timing_system();

    test_env.game.get_world_mut().insert_resource(AxisInputs::new());

    // Create some entities
    let entity1 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. })
        .insert(Controllable { }).id();

    // Run the game
    test_env.game.add_stage_to_schedule(
        "preupdate",
        SystemStage::parallel()
            .with_system(axis_velocity)
    );
    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel()
            .with_system(movement_system)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Horizontal,1.);
    axis_inputs.set(Vertical,1.);

    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    assert_ne!(pos1.x, 0.);
    assert_ne!(pos1.y, 0.);
}

#[test]
fn axis_velocity_without_controllable_component_should_not_change_pos() {
    let mut test_env = initialize_with_entity_and_timing_system();

    test_env.game.get_world_mut().insert_resource(AxisInputs::new());

    // Create some entities
    let entity1 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. }).id();

    // Run the game
    test_env.game.add_stage_to_schedule(
        "preupdate",
        SystemStage::parallel()
            .with_system(axis_velocity)
    );
    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel()
            .with_system(movement_system)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Horizontal,1.);
    axis_inputs.set(Vertical,1.);

    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    assert_eq!(pos1.x, 0.);
    assert_eq!(pos1.y, 0.);
}

#[test]
fn can_add_axis_position_transform_system() {
    let mut test_env = initialize_with_entity_and_timing_system();

    test_env.game.get_world_mut().insert_resource(AxisInputs::new());

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
    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel()
            .with_system(movement_system)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Horizontal,1.);
    axis_inputs.set(Vertical,1.);

    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    assert_ne!(pos1.x, 0.);
    assert_ne!(pos1.y, 0.);
}

#[test]
fn axis_position_transform_system_does_not_work_without_controllable_component() {
    let mut test_env = initialize_with_entity_and_timing_system();

    test_env.game.get_world_mut().insert_resource(AxisInputs::new());

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
    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel()
            .with_system(movement_system)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Horizontal,1.);
    axis_inputs.set(Vertical,1.);

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
    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel()
            .with_system(movement_system)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Horizontal,1.);
    axis_inputs.set(Vertical,1.);

    test_env.game.run_schedule();
    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    assert_eq!(pos1.x, 2.);
    assert_eq!(pos1.y, 2.);
}