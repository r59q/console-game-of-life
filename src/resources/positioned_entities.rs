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

    pub fn put_entity_ref(&mut self, entity_ref: &EntityRef) -> () {
        let position = entity_ref
            .get::<Position>()
            .expect("Could not put entity into PositionedEntities map. No position component!");
        let entity = entity_ref.id();
        self.put_entity(entity, *position)
    }

    pub fn put_entity(&mut self, entity: Entity, position: Position) -> () {
        let x: i32 = position.x as i32;
        let y: i32 = position.y as i32;
        let grid_pos = (x, y);
        self.entities.insert(grid_pos, entity);
    }

    // Todo: Why '&' grid_pos?
    pub fn get(&self, grid_pos: &(i32, i32)) -> Option<&Entity> {
        self.entities.get(grid_pos)
    }

    pub fn get_neighbours_of_position(&self, grid_pos: (i32, i32)) -> Vec<&Entity> {
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

    pub fn clear(&mut self) -> () {
        self.entities.clear();
    }

    pub fn get_all_entities(&self) -> Vec<(&(i32, i32), &Entity)> {
        let mut result = Vec::new();
        for (key, val) in &self.entities {
            result.push((key, val));
        }
        result
    }

    pub fn remove_entity(&mut self, position: (i32, i32)) -> () {
        self.entities.remove(&position);
    }
}

#[cfg(test)]
mod test {
    use std::borrow::{Borrow, BorrowMut};

    use bevy_ecs::{
        entity::EntityLocation,
        prelude::Entity,
        world::{EntityRef, World},
    };

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
        positioned_entities.put_entity_ref(&entity_ref);
    }

    #[test]
    fn mapped_entity_exists_in_map() {
        let mut positioned_entities = PositionedEntities::new();
        let mut test_env = initialize();
        let entity = test_env.game.spawn_prefab(Prefabs::PLACEABLE_CELL);
        let entity_ref = test_env.game.get_world_ref().entity(entity);
        positioned_entities.put_entity_ref(&entity_ref);
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
        let entity1 = place_entity_at(
            &mut test_env,
            center_entity_grid_pos.0 as f64,
            center_entity_grid_pos.1 as f64,
        );
        let entity2 = place_entity_at(&mut test_env, 1., 1.);
        let entity3 = place_entity_at(&mut test_env, 1., 0.);
        let entity4 = place_entity_at(&mut test_env, 0., 1.);
        let entity5 = place_entity_at(&mut test_env, 10., 10.); // Not it's neighbour

        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity1));
        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity2));
        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity3));
        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity4));
        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity5));

        let neighbour_entities =
            positioned_entities.get_neighbours_of_position(center_entity_grid_pos);

        assert_eq!(3, neighbour_entities.len());
    }

    #[test]
    fn can_clear_entities() {
        let mut positioned_entities = PositionedEntities::new();
        let mut test_env = initialize();

        let center_entity_grid_pos = (0, 0);
        let entity1 = place_entity_at(
            &mut test_env,
            center_entity_grid_pos.0 as f64,
            center_entity_grid_pos.1 as f64,
        );
        let entity2 = place_entity_at(&mut test_env, 1., 1.);
        let entity3 = place_entity_at(&mut test_env, 1., 0.);
        let entity4 = place_entity_at(&mut test_env, 0., 1.);
        let entity5 = place_entity_at(&mut test_env, 10., 10.); // Not it's neighbour

        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity1));
        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity2));
        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity3));
        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity4));
        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity5));

        positioned_entities.clear();

        let neighbour_entities =
            positioned_entities.get_neighbours_of_position(center_entity_grid_pos);

        assert_eq!(0, neighbour_entities.len());
    }

    #[test]
    fn can_get_all_entities() {
        let mut positioned_entities = PositionedEntities::new();
        let mut test_env = initialize();

        let center_entity_grid_pos = (0, 0);
        let entity1 = place_entity_at(
            &mut test_env,
            center_entity_grid_pos.0 as f64,
            center_entity_grid_pos.1 as f64,
        );
        let entity2 = place_entity_at(&mut test_env, 0., 1.);
        let entity3 = place_entity_at(&mut test_env, 10., 10.); // Not it's neighbour

        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity1));
        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity2));
        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity3));

        let entities = positioned_entities.get_all_entities();

        assert_eq!(3, entities.len());
    }

    #[test]
    fn can_remove_single_entity() {
        let mut positioned_entities = PositionedEntities::new();
        let mut test_env = initialize();

        let center_entity_grid_pos = (0, 0);
        let entity1 = place_entity_at(
            &mut test_env,
            center_entity_grid_pos.0 as f64,
            center_entity_grid_pos.1 as f64,
        );
        let entity2 = place_entity_at(&mut test_env, 0., 1.);
        let entity3 = place_entity_at(&mut test_env, 10., 10.); // Not it's neighbour

        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity1));
        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity2));
        positioned_entities.put_entity_ref(&get_entity_ref(&test_env, entity3));
        positioned_entities.remove_entity((0, 0));
        let entities = positioned_entities.get_all_entities();

        assert_eq!(2, entities.len());
    }

    fn get_entity_ref(test_env: &TestEnv, entity: Entity) -> EntityRef {
        test_env.game.get_world_ref().entity(entity)
    }

    fn place_entity_at(test_env: &mut TestEnv, x: f64, y: f64) -> Entity {
        test_env
            .game
            .get_world_mut()
            .spawn()
            .insert(Position { x, y })
            .id()
    }
}
