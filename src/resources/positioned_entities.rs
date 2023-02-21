use std::{collections::HashMap, i32};

use bevy_ecs::prelude::Entity;

pub struct PositionedEntities {
    entities: HashMap<(i32, i32), Entity>
}

impl PositionedEntities {
    pub(crate) fn new() -> PositionedEntities {
        return PositionedEntities { entities: HashMap::new() }
    }
}

#[cfg(test)]
mod test {
    use bevy_ecs::world::World;

    use crate::{prefabs::{Prefabs, PrefabCommands}, tests::TestEnv, game::Game};

    use super::PositionedEntities;

    pub fn initialize() -> TestEnv {
        // Not tied to game.
        let entity = World::new().spawn().id();
        let test_env = TestEnv { game: Game::new(1,1,1), entity };
        return test_env;
    }
    
    #[test]
    fn can_create_positioned_entities() {
        PositionedEntities::new();
    }

    #[test]
    fn can_map_entitiy() {
        let positioned_entities = PositionedEntities::new();
        let mut test_env = initialize();
        test_env.game.spawn_prefab(Prefabs::PLACEABLE_CELL);
    }
}