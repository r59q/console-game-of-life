use crate::resources::timer::Timer;
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
