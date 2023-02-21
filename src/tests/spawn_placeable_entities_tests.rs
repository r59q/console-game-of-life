use bevy_ecs::schedule::SystemStage;

use crate::{tests::{initialize_game_paused, initialize}, resources::{inputs::button_inputs::ButtonInputs, pause_state::PauseState}, input_manager::{buttons::Button, self}, systems::spawn_placeables::spawn_placeables, components::{placeable::Placeable, position::Position}, prefabs::Prefabs};

#[test]
fn can_add_spawn_placeable_system() {
    let mut test_env = initialize_game_paused();

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
    let mut test_env = initialize_game_paused();

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
    let mut test_env = initialize_game_paused();

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

#[test]
fn placeable_doesnt_spawn_if_unpaused() {
    let mut test_env = initialize();

    let mut button_inputs = ButtonInputs::new();
    button_inputs.set_btn(Button::Place, input_manager::input_action::InputAction::Down);

    let mut pause_state = PauseState::new();
    pause_state.unpause();
    test_env.game.get_world_mut().insert_resource(pause_state);

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

#[test]
fn placeable_spawns_if_paused() {
    let mut test_env = initialize();

    let mut button_inputs = ButtonInputs::new();
    button_inputs.set_btn(Button::Place, input_manager::input_action::InputAction::Down);

    let mut pause_state = PauseState::new();
    pause_state.pause();
    test_env.game.get_world_mut().insert_resource(pause_state);

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