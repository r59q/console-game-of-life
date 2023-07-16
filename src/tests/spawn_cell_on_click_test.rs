use super::initialize;
use crate::{
    resources::{
        inputs::{button_inputs::ButtonInputs, mouse_inputs::MouseInputs},
        positioned_entities::PositionedEntities,
        view_offset::ViewOffset,
    },
    systems::toggle_cell_on_click::toggle_cell_on_click,
};
use bevy_ecs::schedule::SystemStage;
use console_engine::MouseButton;

use crate::input_manager::input_action::InputAction;

use crate::input_manager::buttons::Button;

#[test]
fn down_input_places_cell() {
    let mut test_env = initialize();

    let mut view_offset = ViewOffset::new();
    view_offset.set_offset(10, 10);

    test_env.game.get_world_mut().insert_resource(view_offset);
    test_env
        .game
        .get_world_mut()
        .insert_resource(PositionedEntities::new());

    let mut button_inputs = ButtonInputs::new();
    button_inputs.set_btn(Button::Fire1, InputAction::Down);

    test_env.game.get_world_mut().insert_resource(button_inputs);

    let mut mouse_inputs = MouseInputs::new();
    mouse_inputs.set_state(MouseButton::Left, InputAction::Down, 1, 1);
    test_env.game.get_world_mut().insert_resource(mouse_inputs);

    test_env.game.add_stage_to_schedule(
        "stage",
        SystemStage::parallel().with_system(toggle_cell_on_click),
    );

    test_env.game.run_schedule();

    let positioned_entities = test_env
        .game
        .get_world_ref()
        .get_resource::<PositionedEntities>()
        .unwrap();

    let test_entity = positioned_entities.get(&(11, 11));

    assert!(test_entity.is_some());
    assert!(positioned_entities.get_all_entities().len() == 1);
}
