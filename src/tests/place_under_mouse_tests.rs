use bevy_ecs::schedule::SystemStage;
use console_engine::MouseButton;

use crate::{
    components::{placeable::Placeable, position::Position},
    input_manager,
    resources::{inputs::mouse_inputs::MouseInputs, view_offset::ViewOffset},
    systems::place_under_mouse::place_under_mouse,
};

use super::initialize;

#[test]
fn can_add_place_under_mouse() {
    let mut test_env = initialize();

    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::single_threaded().with_system(place_under_mouse),
    );

    let stage = test_env
        .game
        .get_schedule_mut()
        .get_stage::<SystemStage>(&"update")
        .unwrap();

    assert!(!matches!(stage.parallel_systems().first(), None))
}

#[test]
fn entity_that_is_placeable_moves_when_mouse_moves() {
    let mut test_env = initialize();

    let mut view_offset = ViewOffset::new();
    view_offset.set_offset(20, 20);
    test_env.game.get_world_mut().insert_resource(view_offset);

    let mut mouse_inputs = MouseInputs::new();
    mouse_inputs.set_state(
        MouseButton::Left,
        input_manager::input_action::InputAction::Down,
        1,
        1,
    );
    test_env.game.get_world_mut().insert_resource(mouse_inputs);

    let placing_entity = test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Placeable { replacement: None })
        .id();

    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel().with_system(place_under_mouse),
    );

    test_env.game.run_schedule();

    let world_entity = test_env
        .game
        .get_world_ref()
        .get_entity(placing_entity)
        .unwrap();
    let pos_x = world_entity.get::<Position>().unwrap().x;
    let pos_y = world_entity.get::<Position>().unwrap().y;
    assert_ne!((0., 0.), (pos_x, pos_y))
}

#[test]
fn entity_that_is_placeable_moves_under_mouse() {
    let mut test_env = initialize();

    let mouse_pos_x = 123;
    let mouse_pos_y = 321;

    let view_offset_x = 20;
    let view_offset_y = 30;

    let expected_pos = (
        (mouse_pos_x + (view_offset_x as u32)).into(),
        (mouse_pos_y + (view_offset_y as u32)).into(),
    );

    let mut mouse_inputs = MouseInputs::new();
    mouse_inputs.set_state(
        MouseButton::Left,
        input_manager::input_action::InputAction::Down,
        mouse_pos_x,
        mouse_pos_y,
    );
    test_env.game.get_world_mut().insert_resource(mouse_inputs);

    let mut view_offset = ViewOffset::new();
    view_offset.set_offset(view_offset_x, view_offset_y);
    test_env.game.get_world_mut().insert_resource(view_offset);

    let placing_entity = test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Placeable { replacement: None })
        .id();

    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel().with_system(place_under_mouse),
    );

    test_env.game.run_schedule();

    let world_entity = test_env
        .game
        .get_world_ref()
        .get_entity(placing_entity)
        .unwrap();
    let pos_x = world_entity.get::<Position>().unwrap().x;
    let pos_y = world_entity.get::<Position>().unwrap().y;

    assert_eq!(expected_pos, (pos_x, pos_y))
}

#[test]
fn entity_that_is_placeable_moves_under_mouse_with_offset() {
    let mut test_env = initialize();

    let mut mouse_inputs = MouseInputs::new();

    let mouse_pos_x = 123;
    let mouse_pos_y = 321;

    let view_offset_x = 20;
    let view_offset_y = 30;

    mouse_inputs.set_state(
        MouseButton::Left,
        input_manager::input_action::InputAction::Down,
        mouse_pos_x,
        mouse_pos_y,
    );
    test_env.game.get_world_mut().insert_resource(mouse_inputs);

    let mut view_offset = ViewOffset::new();
    view_offset.set_offset(view_offset_x, view_offset_y);
    test_env.game.get_world_mut().insert_resource(view_offset);

    let placing_entity = test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Placeable { replacement: None })
        .id();

    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel().with_system(place_under_mouse),
    );

    test_env.game.run_schedule();

    let world_entity = test_env
        .game
        .get_world_ref()
        .get_entity(placing_entity)
        .unwrap();
    let pos_x = world_entity.get::<Position>().unwrap().x;
    let pos_y = world_entity.get::<Position>().unwrap().y;
    let expected_pos = (
        (mouse_pos_x + (view_offset_x as u32)).into(),
        (mouse_pos_y + (view_offset_y as u32)).into(),
    );
    assert_eq!(expected_pos, (pos_x, pos_y))
}

#[test]
fn entity_that_is_not_placeable_does_not_move_under_mouse() {
    let mut test_env = initialize();

    let mut mouse_inputs = MouseInputs::new();
    mouse_inputs.set_state(
        MouseButton::Left,
        input_manager::input_action::InputAction::Down,
        1,
        1,
    );
    test_env.game.get_world_mut().insert_resource(mouse_inputs);

    let mut view_offset = ViewOffset::new();
    view_offset.set_offset(20, 20);
    test_env.game.get_world_mut().insert_resource(view_offset);

    let placing_entity = test_env
        .game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 0., y: 0. })
        .id();

    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel().with_system(place_under_mouse),
    );

    test_env.game.run_schedule();

    let world_entity = test_env
        .game
        .get_world_ref()
        .get_entity(placing_entity)
        .unwrap();
    let pos_x = world_entity.get::<Position>().unwrap().x;
    let pos_y = world_entity.get::<Position>().unwrap().y;
    assert_eq!((0., 0.), (pos_x, pos_y))
}
