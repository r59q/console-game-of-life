use bevy_ecs::system::{Res, ResMut};

use crate::{
    input_manager::buttons::Button,
    resources::{inputs::button_inputs::ButtonInputs, simulation_speed::SimulationSpeed},
};

const SIM_SPEED_CHANGE_DELTA: i32 = 25;

pub fn simulation_speed_changer(
    button_inputs: Res<ButtonInputs>,
    mut sim_speed: ResMut<SimulationSpeed>,
) {
    if button_inputs.get_btn_down(Button::Increase) {
        sim_speed.add_speed(-SIM_SPEED_CHANGE_DELTA);
    }
    if button_inputs.get_btn_down(Button::Decrease) {
        sim_speed.add_speed(SIM_SPEED_CHANGE_DELTA);
    }
}
