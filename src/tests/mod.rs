use bevy_ecs::prelude::*;

use crate::resources::timer::Timer;
use crate::systems::timing::timing_system;

use super::*;

mod ecs_tests;
mod movement_tests;
mod resources_test;

pub struct TestEnv {
    pub game: Game,
    pub entity: Entity,
}

pub fn initialize() -> TestEnv {
    // Not tied to game.
    let entity = World::new().spawn().id();
    return TestEnv { game: Game::new(), entity };
}

pub fn initialize_with_entity() -> TestEnv {
    let mut game = Game::new();
    let entity = game.get_world_mut().spawn().id();
    return TestEnv { game, entity };
}

pub fn initialize_with_entity_and_timing_system() -> TestEnv {
    let mut game = Game::new();
    game.get_world_mut().insert_resource(Timer::new());

    game.get_schedule_mut().add_stage(
        "timing",
        SystemStage::parallel()
            .with_system(timing_system));

    let entity = game.get_world_mut().spawn().id();
    return TestEnv { game, entity };
}


/*
#[test]
fn some_test() {
    let mut test_env = initialize();

    test_env.game.get_world().insert_resource(Timer::default());

    let mut schedule = Schedule::default();
    schedule.add_stage("update", SystemStage::parallel()
        .with_system(movement)
    );

}

 */