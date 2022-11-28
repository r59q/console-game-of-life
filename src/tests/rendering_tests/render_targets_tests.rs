use bevy_ecs::schedule::SystemStage;
use console_engine::MouseButton;

use crate::{tests::initialize, resources::{inputs::mouse_inputs::MouseInputs, view_offset::ViewOffset}, input_manager::input_action::InputAction, systems::drag_view_offset::drag_view_offset};


#[test]
fn mouse_inputs_changes_offset() {
    let mut test_env = initialize();

    let view_offset = ViewOffset::new();
    let mut mouse_inputs = MouseInputs::new();
    mouse_inputs.set_state(MouseButton::Left, InputAction::Down, 0, 0);
    mouse_inputs.set_state(MouseButton::Left, InputAction::Drag, 1, 1);

    test_env.game.get_world_mut().insert_resource::<ViewOffset>(view_offset);
    test_env.game.get_world_mut().insert_resource::<MouseInputs>(mouse_inputs);

    test_env.game.add_stage_to_schedule(
        "move", 
        SystemStage::single_threaded()
            .with_system(drag_view_offset));

    test_env.game.run_schedule();
    
    let drag_delta = test_env.game.get_world_mut().get_resource_mut::<MouseInputs>().unwrap().get_drag_delta();
    
    assert_ne!(drag_delta, (0,0));

    let fetched_offset = test_env.game.get_world_ref().get_resource::<ViewOffset>().unwrap();

    assert_ne!(fetched_offset.get_offset() , (0,0))
}


#[test]
fn mouse_inputs_changes_iteratively_offset() {
    let mut test_env = initialize();

    let view_offset = ViewOffset::new();
    let mut mouse_inputs = MouseInputs::new();
    mouse_inputs.set_state(MouseButton::Left, InputAction::Down, 0, 0);
    mouse_inputs.set_state(MouseButton::Left, InputAction::Drag, 1, 1);

    test_env.game.get_world_mut().insert_resource::<ViewOffset>(view_offset);
    test_env.game.get_world_mut().insert_resource::<MouseInputs>(mouse_inputs);

    test_env.game.add_stage_to_schedule(
        "move", 
        SystemStage::single_threaded()
            .with_system(drag_view_offset));

    test_env.game.run_schedule();
    
    let drag_delta = test_env.game.get_world_mut().get_resource_mut::<MouseInputs>().unwrap().get_drag_delta();
    
    assert_ne!(drag_delta, (0,0));

    test_env.game.run_schedule();

    test_env.game.get_world_mut().get_resource_mut::<MouseInputs>().unwrap().set_state(MouseButton::Left, InputAction::Drag, 2, 2);

    let drag_delta_new = test_env.game.get_world_mut().get_resource_mut::<MouseInputs>().unwrap().get_drag_delta();

    assert_ne!(drag_delta, drag_delta_new);

    let fetched_offset = test_env.game.get_world_ref().get_resource::<ViewOffset>().unwrap();

    assert_ne!(fetched_offset.get_offset() , (0,0))
}

