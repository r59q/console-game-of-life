use bevy_ecs::{prelude::Component, world::World};

use crate::components::position::Position;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

#[derive(Component)]
struct TestComponent {x: i32}

#[test]
fn can_add_component() {
    let mut test_env = initialize();

    let targetx = 1337;
    let world: &mut World = test_env.game.get_world();

    let entity = world.spawn().insert(TestComponent {x:targetx}).id();

    let entity_ref = world.entity(entity);
    let test_component_x = entity_ref.get::<TestComponent>().unwrap().x;

    assert_eq!(test_component_x, targetx)
}

#[test]
fn can_add_entity() {
    let mut test_env = initialize();

    let world: &mut World = test_env.game.get_world();

    assert_eq!(0, world.entities().len());

    let entity = world.spawn().id();

    assert_eq!(1, world.entities().len());
}

#[test]
fn can_add_position_component() {
    let mut test_env = initialize_with_entity();

    let entity = test_env.entity;
    let pos_to_add = Position {x:1., y:1.};
    test_env.game.get_world().entity_mut(entity).insert(pos_to_add).id();

    let pos = test_env.game.get_world().entity(entity).get::<Position>().unwrap();
    let pos_x = pos.x;
    let pos_y = pos.x;
    let pos_to_add_x = pos_to_add.x;
    let pos_to_add_y = pos_to_add.y;

    assert_eq!(pos_x, pos_to_add_x);
    assert_eq!(pos_y, pos_to_add_y);    
}
