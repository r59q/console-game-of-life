use bevy_ecs::schedule::SystemStage;

use crate::{tests::initialize, systems::drag_view_offset::drag_view_offset, resources::{view_offset::ViewOffset, inputs::mouse_inputs::MouseInputs}};

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
