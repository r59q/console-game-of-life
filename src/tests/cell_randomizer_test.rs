use bevy_ecs::schedule::SystemStage;

use crate::{
    components::position::Position,
    input_manager::{buttons::Button, input_action::InputAction},
    resources::{inputs::button_inputs::ButtonInputs, positioned_entities::PositionedEntities},
    systems::cell_randomizer::cell_randomizer,
};

use super::initialize;

#[test]
fn randomizing_adds_cells_to_positioned_entities() {
    let mut test_env = initialize();
    let mut button_inputs = ButtonInputs::new();
    button_inputs.set_btn(Button::Random, InputAction::Down);

    test_env.game.get_world_mut().insert_resource(button_inputs);
    test_env
        .game
        .get_world_mut()
        .insert_resource(PositionedEntities::new());

    test_env.game.add_stage_to_schedule(
        "stage",
        SystemStage::parallel().with_system(cell_randomizer),
    );

    test_env.game.run_schedule();

    let positioned_entities = test_env
        .game
        .get_world_ref()
        .get_resource::<PositionedEntities>()
        .unwrap();

    assert!(positioned_entities.get_all_entities().len() > 0);
}
