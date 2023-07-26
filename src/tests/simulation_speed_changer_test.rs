use bevy_ecs::schedule::SystemStage;

use crate::{
    input_manager::{buttons::Button, input_action::InputAction},
    resources::{inputs::button_inputs::ButtonInputs, simulation_speed::SimulationSpeed},
    systems::simulation_speed_changer::{self, simulation_speed_changer},
};

use super::initialize;

#[test]
fn can_increase_simulation_speed() {
    let mut test_env = initialize();

    let mut button_inputs = ButtonInputs::new();
    button_inputs.set_btn(Button::Increase, InputAction::Down);
    test_env.game.get_world_mut().insert_resource(button_inputs);
    test_env
        .game
        .get_world_mut()
        .insert_resource(SimulationSpeed::new(500));
    test_env.game.add_stage_to_schedule(
        "stage",
        SystemStage::parallel().with_system(simulation_speed_changer),
    );

    test_env.game.run_schedule();

    let sim_speed = test_env
        .game
        .get_world_ref()
        .get_resource::<SimulationSpeed>()
        .unwrap();

    assert!(sim_speed.get_speed() < 500);
}

#[test]
fn can_decrease_simulation_speed() {
    let mut test_env = initialize();

    let mut button_inputs = ButtonInputs::new();
    button_inputs.set_btn(Button::Decrease, InputAction::Down);
    test_env.game.get_world_mut().insert_resource(button_inputs);
    test_env
        .game
        .get_world_mut()
        .insert_resource(SimulationSpeed::new(500));
    test_env.game.add_stage_to_schedule(
        "stage",
        SystemStage::parallel().with_system(simulation_speed_changer),
    );

    test_env.game.run_schedule();

    let sim_speed = test_env
        .game
        .get_world_ref()
        .get_resource::<SimulationSpeed>()
        .unwrap();

    assert!(sim_speed.get_speed() > 500);
}
