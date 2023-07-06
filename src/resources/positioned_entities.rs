use std::{collections::HashMap, i32};

use bevy_ecs::{prelude::Entity, world::EntityRef};

use crate::components::position::Position;

pub struct PositionedEntities {
    entities: HashMap<(i32, i32), Entity>,
}

impl PositionedEntities {
    pub(crate) fn new() -> PositionedEntities {
        return PositionedEntities {
            entities: HashMap::new(),
        };
    }

    fn put(&mut self, entity_ref: &EntityRef) -> () {
        let position = entity_ref
            .get::<Position>()
            .expect("Could not put entity into PositionedEntities map. No position component!");
        let x: i32 = position.x as i32;
        let y: i32 = position.y as i32;
        let grid_pos = (x, y);
        let entity = entity_ref.id();
        self.entities.insert(grid_pos, entity);
    }

    fn get(&self, grid_pos: &(i32, i32)) -> Option<&Entity> {
        self.entities.get(grid_pos)
    }

    fn get_neighbours_of_position(&self, grid_pos: (i32, i32)) -> Vec<&Entity> {
        let mut result = Vec::<&Entity>::new();
        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                let current_pos = (grid_pos.0 + x_offset, grid_pos.1 + y_offset);
                if current_pos != grid_pos {
                    let neighbour_option = self.get(&current_pos);
                    if let Some(neighbour) = neighbour_option {
                        result.push(neighbour);
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use std::borrow::{Borrow, BorrowMut};

    use bevy_ecs::{entity::EntityLocation, prelude::Entity, world::World};

    use crate::{
        components::position::Position,
        game::Game,
        prefabs::{PrefabCommands, Prefabs},
        tests::TestEnv,
    };

    use super::PositionedEntities;

    pub fn initialize() -> TestEnv {
        // Not tied to game.
        let entity = World::new().spawn().id();
        let test_env = TestEnv {
            game: Game::new(1, 1, 1),
            entity,
        };
        return test_env;
    }

    #[test]
    fn can_create_positioned_entities() {
        PositionedEntities::new();
    }

    #[test]
    fn can_map_entitiy() {
        let mut positioned_entities = PositionedEntities::new();
        let mut test_env = initialize();
        let entity = test_env.game.spawn_prefab(Prefabs::PLACEABLE_CELL);
        let entity_ref = test_env.game.get_world_ref().entity(entity);
        positioned_entities.put(&entity_ref);
    }

    #[test]
    fn mapped_entity_exists_in_map() {
        let mut positioned_entities = PositionedEntities::new();
        let mut test_env = initialize();
        let entity = test_env.game.spawn_prefab(Prefabs::PLACEABLE_CELL);
        let entity_ref = test_env.game.get_world_ref().entity(entity);
        positioned_entities.put(&entity_ref);
        let position = entity_ref.get::<Position>().unwrap();
        let grid_pos = (position.x as i32, position.y as i32);
        let mapped_entity = positioned_entities.get(&grid_pos);

        assert_ne!(mapped_entity, None);
        assert_eq!(mapped_entity.expect("Error").clone(), entity);
    }

    #[test]
    fn can_get_neighbours_of_entity() {
        let mut positioned_entities = PositionedEntities::new();
        let mut test_env = initialize();

        let center_entity_grid_pos = (0, 0);
        place_entity_at(
            &mut test_env,
            &mut positioned_entities,
            center_entity_grid_pos.0 as f64,
            center_entity_grid_pos.1 as f64,
        );
        place_entity_at(&mut test_env, &mut positioned_entities, 1., 1.);
        place_entity_at(&mut test_env, &mut positioned_entities, 1., 0.);
        place_entity_at(&mut test_env, &mut positioned_entities, 0., 1.);
        let neighbour_entities =
            positioned_entities.get_neighbours_of_position(center_entity_grid_pos);

        assert_ne!(0, neighbour_entities.len());
        assert_eq!(3, neighbour_entities.len());
    }

    fn place_entity_at(
        test_env: &mut TestEnv,
        positioned_entities: &mut PositionedEntities,
        x: f64,
        y: f64,
    ) -> Entity {
        let entity = test_env
            .game
            .get_world_mut()
            .spawn()
            .insert(Position { x, y })
            .id();
        positioned_entities.put(&test_env.game.get_world_ref().entity(entity));
        entity
    }
}
