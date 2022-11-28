use console_engine::MouseButton;

use crate::components::controllable::Controllable;
use crate::components::placeable::{Placeable, self};
use crate::components::position::Position;
use crate::components::velocity::Velocity;
use crate::input_manager::buttons::Button;
use crate::resources::inputs::mouse_inputs;
use crate::resources::pause_state::PauseState;
use crate::resources::timer::Timer;
use crate::systems::axis_position_transform::axis_position_transform;
use crate::systems::axis_velocity::axis_velocity;
use crate::systems::movement::movement_system;
use crate::systems::place_under_mouse::place_under_mouse;
use crate::systems::spawn_placeables::spawn_placeables;
use crate::systems::timing::timing_system;

use super::*;

#[test]
fn can_add_movement_system() {
    let mut test_env = initialize();
    test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. });

    let mut schedule = Schedule::default();

    schedule.add_stage(
        "update",
        SystemStage::parallel().
            with_system(movement_system),
    );
}

#[test]
fn no_velocity_does_not_change_position() {
    let mut test_env = initialize();
    
    test_env.game.get_world_mut().insert_resource(PauseState::new());
    
    let before_x = 0.;
    let before_y = 0.;

    let velocity_x = 0.;
    let velocity_y = 0.;
    let entity = test_env.game.get_world_mut().spawn()
        .insert(Position { x: before_x, y: before_y })
        .insert(Velocity { x: velocity_x, y: velocity_y }).id();

    let mut schedule = Schedule::default();

    let timer = Timer::new();

    test_env.game.get_world_mut().insert_resource(timer);

    schedule.add_stage("timing", SystemStage::parallel().with_system(timing_system));

    schedule.add_stage(
        "update",
        SystemStage::parallel().
            with_system(movement_system),
    );

    schedule.run(test_env.game.get_world_mut());

    let pos = test_env.game.get_world_mut().entity(entity).get::<Position>().unwrap();

    assert_eq!(pos.x, before_x);
    assert_eq!(pos.y, before_y);
}

#[test]
fn some_velocity_does_change_position() {
    let mut test_env = initialize();
    test_env.game.get_world_mut().insert_resource(PauseState::new());


    let before_x = 0.;
    let before_y = 0.;

    let velocity_x = 2.;
    let velocity_y = 2.;
    let entity = test_env.game.get_world_mut().spawn()
        .insert(Position { x: before_x, y: before_y })
        .insert(Velocity { x: velocity_x, y: velocity_y }).id();

    let mut schedule = Schedule::default();

    let timer = Timer::new();

    test_env.game.get_world_mut().insert_resource(timer);

    schedule.add_stage("timing", SystemStage::parallel().with_system(timing_system));

    schedule.add_stage(
        "update",
        SystemStage::parallel().
            with_system(movement_system),
    );

    schedule.run(test_env.game.get_world_mut());

    let pos = test_env.game.get_world_mut().entity(entity).get::<Position>().unwrap();

    assert_ne!(pos.x, before_x);
    assert_ne!(pos.y, before_y);
}

#[test]
fn different_velocities_makes_different_changes() {
    let mut test_env = initialize_with_entity_and_timing_system();
    test_env.game.get_world_mut().insert_resource(PauseState::new());

    // Create some entities
    let entity1 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 1., y: 1. }).id();
    let entity2 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 2., y: 2. }).id();

    // Run the game
    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel()
            .with_system(movement_system)
    );
    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    let pos2 =  game.get_world_ref().entity(entity2).get::<Position>().unwrap();
    assert_ne!(pos1.x, 0.);
    assert_ne!(pos1.y, 0.);
    assert_ne!(pos2.x, 0.);
    assert_ne!(pos2.y, 0.);
    assert_ne!(pos1.x, pos2.x);
    assert_ne!(pos1.y, pos2.y);
}

#[test]
fn can_add_axis_velocity_system() {
    let mut test_env = initialize_with_entity_and_timing_system();

    test_env.game.get_world_mut().insert_resource(AxisInputs::new());
    test_env.game.get_world_mut().insert_resource(PauseState::new());

    // Create some entities
    let entity1 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. })
        .insert(Controllable { }).id();

    // Run the game
    test_env.game.add_stage_to_schedule(
        "preupdate",
        SystemStage::parallel()
            .with_system(axis_velocity)
    );
    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel()
            .with_system(movement_system)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Horizontal,1.);
    axis_inputs.set(Vertical,1.);

    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    assert_ne!(pos1.x, 0.);
    assert_ne!(pos1.y, 0.);
}

#[test]
fn axis_velocity_without_controllable_component_should_not_change_pos() {
    let mut test_env = initialize_with_entity_and_timing_system();

    test_env.game.get_world_mut().insert_resource(AxisInputs::new());
    test_env.game.get_world_mut().insert_resource(PauseState::new());

    // Create some entities
    let entity1 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. }).id();

    // Run the game
    test_env.game.add_stage_to_schedule(
        "preupdate",
        SystemStage::parallel()
            .with_system(axis_velocity)
    );
    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel()
            .with_system(movement_system)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Horizontal,1.);
    axis_inputs.set(Vertical,1.);

    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    assert_eq!(pos1.x, 0.);
    assert_eq!(pos1.y, 0.);
}

#[test]
fn can_add_axis_position_transform_system() {
    let mut test_env = initialize_with_entity_and_timing_system();

    test_env.game.get_world_mut().insert_resource(AxisInputs::new());
    test_env.game.get_world_mut().insert_resource(PauseState::new());

    // Create some entities
    let entity1 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. })
        .insert(Controllable { }).id();

    // Run the game
    test_env.game.add_stage_to_schedule(
        "preupdate",
        SystemStage::parallel()
            .with_system(axis_position_transform)
    );
    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel()
            .with_system(movement_system)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Horizontal,1.);
    axis_inputs.set(Vertical,1.);

    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    assert_ne!(pos1.x, 0.);
    assert_ne!(pos1.y, 0.);
}

#[test]
fn cannot_move_with_axis_transform_system_when_paused() {
    let mut test_env = initialize_with_entity_and_timing_system();

    test_env.game.get_world_mut().insert_resource(AxisInputs::new());
    let mut pause_state = PauseState::new();
    pause_state.pause();
    test_env.game.get_world_mut().insert_resource(pause_state);

    // Create some entities
    let entity1 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. })
        .insert(Controllable { }).id();

    // Run the game
    test_env.game.add_stage_to_schedule(
        "preupdate",
        SystemStage::parallel()
            .with_system(axis_position_transform)
    );
    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel()
            .with_system(movement_system)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Horizontal,1.);
    axis_inputs.set(Vertical,1.);

    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    assert_eq!(pos1.x, 0.);
    assert_eq!(pos1.y, 0.);
}

#[test]
fn axis_position_transform_system_does_not_work_without_controllable_component() {
    let mut test_env = initialize_with_entity_and_timing_system();

    test_env.game.get_world_mut().insert_resource(AxisInputs::new());
    test_env.game.get_world_mut().insert_resource(PauseState::new());

    // Create some entities
    let entity1 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. }).id();

    // Run the game
    test_env.game.add_stage_to_schedule(
        "preupdate",
        SystemStage::parallel()
            .with_system(axis_position_transform)
    );
    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel()
            .with_system(movement_system)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Horizontal,1.);
    axis_inputs.set(Vertical,1.);

    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    assert_eq!(pos1.x, 0.);
    assert_eq!(pos1.y, 0.);
}

#[test]
fn moves_multiple_times_with_transform_system() {
    let mut test_env = initialize_with_entity_and_timing_system();

    test_env.game.get_world_mut().insert_resource(AxisInputs::new());
    test_env.game.get_world_mut().insert_resource(PauseState::new());

    // Create some entities
    let entity1 = test_env.game.get_world_mut().spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 0., y: 0. })
        .insert(Controllable { }).id();

    // Run the game
    test_env.game.add_stage_to_schedule(
        "preupdate",
        SystemStage::parallel()
            .with_system(axis_position_transform)
    );
    test_env.game.add_stage_to_schedule(
        "update",
        SystemStage::parallel()
            .with_system(movement_system)
    );

    let mut axis_inputs = test_env.game.get_world_mut().get_resource_mut::<AxisInputs>().unwrap();

    axis_inputs.set(Horizontal,1.);
    axis_inputs.set(Vertical,1.);

    test_env.game.run_schedule();
    test_env.game.run_schedule();

    let game = test_env.game;
    let pos1 = game.get_world_ref().entity(entity1).get::<Position>().unwrap();
    assert_eq!(pos1.x, 2.);
    assert_eq!(pos1.y, 2.);
}

#[test]
fn can_pause_movement() {
    let mut test_env = initialize_with_entity_and_timing_system();
    
    let moving_entity = test_env.game.get_world_mut().spawn()
        .insert(Position {x:0., y:0.})
        .insert(Velocity {x:1., y:1.}).id();

    let pause_resource = PauseState::new();
    test_env.game.get_world_mut().insert_resource(pause_resource);
    test_env.game.add_stage_to_schedule(
        "move",
        SystemStage::parallel()
            .with_system(movement_system)
    );

    test_env.game.run_schedule();

    
    let moved_position1 = test_env.game.get_world_ref()
        .get_entity(moving_entity)
        .unwrap().get::<Position>().unwrap();
    
    let moved_pos_x1 = moved_position1.x;
    let moved_pos_y1 = moved_position1.y;

    assert_ne!(moved_pos_x1, 0.);
    assert_ne!(moved_pos_y1, 0.);
        
    let mut pause_state = test_env.game.get_world_mut().get_resource_mut::<PauseState>().unwrap();

    pause_state.pause();
    
    test_env.game.run_schedule();

    let moved_position2 = test_env.game.get_world_ref()
        .get_entity(moving_entity)
        .unwrap().get::<Position>().unwrap();
    
    let moved_pos_x2 = moved_position2.x;
    let moved_pos_y2 = moved_position2.y;
    
    assert_eq!(moved_pos_y1, moved_pos_y2);
    assert_eq!(moved_pos_x1, moved_pos_x2);
}

#[test]
fn can_add_place_under_mouse() {
    let mut test_env = initialize();

    test_env.game.add_stage_to_schedule("update", SystemStage::parallel()
        .with_system(place_under_mouse));

    let stage = test_env.game.get_schedule_mut().get_stage::<SystemStage>(&"update").unwrap();

    assert!(!matches!(stage.parallel_systems().first(), None))
}

#[test]
fn entity_that_is_placeable_moves_when_mouse_moves() {
    let mut test_env = initialize();


    let mut view_offset = ViewOffset::new();
    view_offset.set_offset(20, 20);
    test_env.game.get_world_mut().insert_resource(view_offset);

    let mut mouse_inputs = MouseInputs::new();
    mouse_inputs.set_state(MouseButton::Left, input_manager::input_action::InputAction::Down, 1, 1);
    test_env.game.get_world_mut().insert_resource(mouse_inputs);

    let placing_entity = test_env.game.get_world_mut().spawn()
        .insert(Position {x:0., y:0.})
        .insert(Placeable { replacement: None }).id();

    test_env.game.add_stage_to_schedule("update", SystemStage::parallel()
        .with_system(place_under_mouse));
    
    test_env.game.run_schedule();

    let world_entity = test_env.game.get_world_ref().get_entity(placing_entity).unwrap();
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

    let expected_pos = ((mouse_pos_x + (view_offset_x as u32)).into(), (mouse_pos_y + (view_offset_y as u32)).into());

    let mut mouse_inputs = MouseInputs::new();
    mouse_inputs.set_state(MouseButton::Left, input_manager::input_action::InputAction::Down, mouse_pos_x, mouse_pos_y);
    test_env.game.get_world_mut().insert_resource(mouse_inputs);

    let mut view_offset = ViewOffset::new();
    view_offset.set_offset(view_offset_x, view_offset_y);
    test_env.game.get_world_mut().insert_resource(view_offset);
    
    let placing_entity = test_env.game.get_world_mut().spawn()
        .insert(Position {x:0., y:0.})
        .insert(Placeable { replacement: None }).id();

    test_env.game.add_stage_to_schedule("update", SystemStage::parallel()
        .with_system(place_under_mouse));
    
    test_env.game.run_schedule();

    let world_entity = test_env.game.get_world_ref().get_entity(placing_entity).unwrap();
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

    mouse_inputs.set_state(MouseButton::Left, input_manager::input_action::InputAction::Down, mouse_pos_x, mouse_pos_y);
    test_env.game.get_world_mut().insert_resource(mouse_inputs);

    let mut view_offset = ViewOffset::new();
    view_offset.set_offset(view_offset_x, view_offset_y);
    test_env.game.get_world_mut().insert_resource(view_offset);

    let placing_entity = test_env.game.get_world_mut().spawn()
        .insert(Position {x:0., y:0.})
        .insert(Placeable { replacement: None }).id();

    test_env.game.add_stage_to_schedule("update", SystemStage::parallel()
        .with_system(place_under_mouse));
    
    test_env.game.run_schedule();

    let world_entity = test_env.game.get_world_ref().get_entity(placing_entity).unwrap();
    let pos_x = world_entity.get::<Position>().unwrap().x;
    let pos_y = world_entity.get::<Position>().unwrap().y;
    let expected_pos = ((mouse_pos_x + (view_offset_x as u32)).into(), (mouse_pos_y + (view_offset_y as u32)).into());
    assert_eq!(expected_pos, (pos_x, pos_y))
}

#[test]
fn entity_that_is_not_placeable_does_not_move_under_mouse() {
    let mut test_env = initialize();

    let mut mouse_inputs = MouseInputs::new();
    mouse_inputs.set_state(MouseButton::Left, input_manager::input_action::InputAction::Down, 1, 1);
    test_env.game.get_world_mut().insert_resource(mouse_inputs);

    let mut view_offset = ViewOffset::new();
    view_offset.set_offset(20, 20);
    test_env.game.get_world_mut().insert_resource(view_offset);

    let placing_entity = test_env.game.get_world_mut().spawn()
        .insert(Position {x:0., y:0.}).id();

    test_env.game.add_stage_to_schedule("update", SystemStage::parallel()
        .with_system(place_under_mouse));
    
    test_env.game.run_schedule();

    let world_entity = test_env.game.get_world_ref().get_entity(placing_entity).unwrap();
    let pos_x = world_entity.get::<Position>().unwrap().x;
    let pos_y = world_entity.get::<Position>().unwrap().y;
    assert_eq!((0., 0.), (pos_x, pos_y))
}

#[test]
fn can_add_spawn_placeable_system() {
    let mut test_env = initialize();

    let mut button_inputs = ButtonInputs::new();
    button_inputs.set_btn(Button::Place, input_manager::input_action::InputAction::Down);
    test_env.game.get_world_mut().insert_resource(button_inputs);

    let placing_entity = test_env.game.get_world_mut().spawn()
        .insert(Position {x:0., y:0.})
        .insert(Placeable { replacement: None }).id();

    test_env.game.add_stage_to_schedule("update", SystemStage::parallel()
        .with_system(spawn_placeables));
    
    test_env.game.run_schedule();

    let world_entity = test_env.game.get_world_ref().get_entity(placing_entity);

    assert!(!matches!(world_entity, None));
}

#[test]
fn placeable_gets_spawned() {
    let mut test_env = initialize();

    let mut button_inputs = ButtonInputs::new();
    button_inputs.set_btn(Button::Place, input_manager::input_action::InputAction::Down);
    test_env.game.get_world_mut().insert_resource(button_inputs);


    test_env.game.get_world_mut().spawn()
        .insert(Position {x:0., y:0.})
        .insert(Placeable { replacement: Some(Prefabs::CELL) });

    test_env.game.add_stage_to_schedule("update", SystemStage::parallel()
        .with_system(spawn_placeables));
    
    test_env.game.run_schedule();

    let entity_count = test_env.game.get_world_ref().entities().len();

    assert_eq!(entity_count, 2);
}

#[test]
fn placeable_doesnt_spawn_if_action_not_down() {
    let mut test_env = initialize();

    let mut button_inputs = ButtonInputs::new();
    button_inputs.set_btn(Button::Place, input_manager::input_action::InputAction::Up);
    test_env.game.get_world_mut().insert_resource(button_inputs);


    test_env.game.get_world_mut().spawn()
        .insert(Position {x:0., y:0.})
        .insert(Placeable { replacement: Some(Prefabs::CELL) });

    test_env.game.add_stage_to_schedule("update", SystemStage::parallel()
        .with_system(spawn_placeables));
    
    test_env.game.run_schedule();

    let entity_count = test_env.game.get_world_ref().entities().len();

    assert_eq!(entity_count, 1);
}