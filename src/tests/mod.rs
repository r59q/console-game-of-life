mod ecs_tests;
use bevy_ecs::prelude::*;

use super::*;

pub struct TestEnv {
    pub game: Game,
    pub entity: Entity
}

pub fn initialize() -> TestEnv {
    // Not tied to game.
    let entity = World::new().spawn().id();
    return TestEnv { game: Game::new(), entity: entity }
}

pub fn initialize_with_entity() -> TestEnv {
    let mut game = Game::new();
    let entity = game.get_world().spawn().id();
    return TestEnv { game: game, entity: entity }
}