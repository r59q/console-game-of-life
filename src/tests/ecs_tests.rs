use bevy_ecs::{prelude::{Component, Entity}, world::World, system::Query, schedule::{Schedule, SystemStage}};

use crate::components::{position::Position, velocity::Velocity};

use super::*;

#[derive(Component)]
struct TestComponent {x: i32}

fn test_system (mut query: Query<(Entity, &mut TestComponent)>) {
    for (_entity, mut test) in &mut query {
        test.x +=1;
    }
}

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

    world.spawn();

    assert_eq!(1, world.entities().len());
}

#[test]
fn can_add_position_component() {
    let mut test_env = initialize_with_entity();

    let entity = test_env.entity;
    let pos_to_add = Position {x:1., y:1.};
    test_env.game.get_world().entity_mut(entity).insert(pos_to_add);

    let pos = test_env.game.get_world().entity(entity).get::<Position>().unwrap();
    let pos_x = pos.x;
    let pos_y = pos.y;
    let pos_to_add_x = pos_to_add.x;
    let pos_to_add_y = pos_to_add.y;

    assert_eq!(pos_x, pos_to_add_x);
    assert_eq!(pos_y, pos_to_add_y);    
}

#[test]
fn can_add_system() {
    let mut test_env = initialize_with_entity();
    let mut schedule = Schedule::default();

    schedule.add_stage(
        "update", 
        SystemStage::parallel().
            with_system(test_system)
    );

    schedule.run_once(&mut test_env.game.get_world())
}

#[test]
fn system_affects_component_values() {
    let mut test_env = initialize_with_entity();
    let entity = test_env.game.get_world().spawn().insert(TestComponent{x:1}).id();

    let mut schedule = Schedule::default();

    schedule.add_stage(
        "update", 
        SystemStage::parallel().
            with_system(test_system)
    );

    let mut comp = test_env.game.get_world().get_entity(entity).unwrap().get::<TestComponent>().unwrap();
    let prev_x = comp.x;
    schedule.run_once(&mut test_env.game.get_world());
    comp = test_env.game.get_world().get_entity(entity).unwrap().get::<TestComponent>().unwrap();
    assert_eq!(prev_x, comp.x -1)
}

#[test]
fn can_add_velocity_component() {
    let mut test_env = initialize_with_entity();

    let entity = test_env.entity;
    let velocity_to_add = Velocity {x:6., y:-51.};
    test_env.game.get_world().entity_mut(entity).insert(velocity_to_add);

    let velo = test_env.game.get_world().entity(entity).get::<Velocity>().unwrap();
    let velo_x = velo.x;
    let velo_y = velo.y;
    let velo_to_add_x = velocity_to_add.x;
    let velo_to_add_y = velocity_to_add.y;

    assert_eq!(velo_x, velo_to_add_x);
    assert_eq!(velo_y, velo_to_add_y);
}

#[test]
fn can_add_multiple_components() {
    let mut test_env = initialize_with_entity();

    let entity = test_env.entity;
    let velocity_component = Velocity {x:6., y:-51.};
    let position_component = Position {x:6., y:-51.};
    test_env.game.get_world().entity_mut(entity)
        .insert(velocity_component)
        .insert(position_component);

    let world_entity = test_env.game.get_world().entity(entity);

    let world_position = world_entity.get::<Position>();
    let world_velocity = world_entity.get::<Position>();

    assert_ne!(world_velocity.is_none(), true);
    assert_ne!(world_position.is_none(), true);
}