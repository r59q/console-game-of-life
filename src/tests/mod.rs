use bevy_ecs::{prelude::Entity, world::{EntityMut, World}};

use crate::game::{Game};

#[cfg(test)]
mod ecs_tests;
struct TestEnv {
    pub game: Game,
    pub entity: Entity
}

fn initialize() -> TestEnv {
    // Not tied to game.
    let entity = World::new().spawn().id();
    return TestEnv { game: Game::new(), entity: entity }
}
 
fn initialize_with_entity() -> TestEnv {
    let mut game = Game::new();
    let entity = game.get_world().spawn().id();
    return TestEnv { game: game, entity: entity }
}