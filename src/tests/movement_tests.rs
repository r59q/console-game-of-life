use super::*;

use crate::systems::movement::movement_system;

#[test]
fn can_add_movement_system() {
    let mut test_env = initialize();
    test_env.game.get_world().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. });

    let mut schedule = Schedule::default();

    schedule.add_stage(
        "update",
        SystemStage::parallel().
            with_system(movement_system)
    );
}

/*

#[test]
fn movement_changes_position() {
    let mut test_env = initialize();
    let before_x =  0.;
    let before_y =  0.;

    let velo_x = 0.;
    let velo_y = 0.;
    test_env.game.get_world().spawn()
        .insert(Position { x: before_x, y: before_y })
        .insert(Velocity { x: velo_x, y: velo_y }).id();

    let mut schedule = Schedule::default();

    schedule.add_stage(
        "update",
        SystemStage::parallel().
            with_system(movement_system)
    );
}*/

