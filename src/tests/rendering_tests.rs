use console_engine::MouseButton;

use crate::components::rendering_character::RenderingCharacter;
use crate::input_manager::input_action::InputAction;
use crate::resources::view_offset;
use crate::systems::character_renderer::character_renderer;
use crate::systems::drag_view_offset::drag_view_offset;

use super::*;

#[test]
fn can_add_rendering_character_component() {
    let mut test_env = initialize();

    test_env.game.get_world_mut().spawn()
        .insert(RenderingCharacter { character: '#' });
}

#[test]
fn can_get_rendering_character_component() {
    let mut test_env = initialize();

    let entity = test_env.game.get_world_mut().spawn()
        .insert(RenderingCharacter { character: '#' }).id();

    let render_char = test_env.game.get_world_ref().entity(entity).get::<RenderingCharacter>();

    match render_char {
        None => { assert!(false) }
        Some(r_c) => { assert_eq!(r_c.character, '#') }
    }
}

#[test]
fn can_add_renderer() {
    let mut test_env = initialize();
    test_env.game.add_stage_to_schedule("Update", SystemStage::parallel()
        .with_system(character_renderer),
    )
}

#[test]
fn add_and_reset_render_target() {
    let mut test_env = initialize_with_rendered_entity_and_timing_system();

    let mut render_targets = test_env.game.get_world_mut().get_resource::<RenderTargets>();

    match render_targets {
        None => { assert!(false) }
        Some(targets) => {
            assert_eq!(0, targets.get_cloned_targets().len())
        }
    }

    test_env.game.run_schedule();
    test_env.game.run_schedule();
    test_env.game.run_schedule();

    render_targets = test_env.game.get_world_mut().get_resource::<RenderTargets>();

    match render_targets {
        None => { assert!(false) }
        Some(targets) => {
            assert_eq!(1, targets.get_cloned_targets().len())
        }
    }
}


#[test]
fn can_add_view_offset_dragging_system() {
    let mut test_env = initialize();

    test_env.game.add_stage_to_schedule(
        "move", 
        SystemStage::single_threaded()
            .with_system(drag_view_offset));
}


#[test]
fn can_add_and_run_view_offset_dragging_system() {
    let mut test_env = initialize();

    let view_offset = ViewOffset::new();
    let mouse_inputs = MouseInputs::new();

    test_env.game.get_world_mut().insert_resource::<ViewOffset>(view_offset);
    test_env.game.get_world_mut().insert_resource::<MouseInputs>(mouse_inputs);

    test_env.game.add_stage_to_schedule(
        "move", 
        SystemStage::single_threaded()
            .with_system(drag_view_offset));

    test_env.game.run_schedule();
}




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

