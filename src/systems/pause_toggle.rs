use bevy_ecs::system::{Res, ResMut};

use crate::{resources::{inputs::{button_inputs::ButtonInputs}, pause_state::PauseState}, input_manager::buttons::Button};

pub fn pause_toggle(button_inputs: Res<ButtonInputs>, mut pause_state: ResMut<PauseState>) {
    let did_press_pause = button_inputs.get_btn_down(Button::Pause);
    if did_press_pause {
        if pause_state.is_paused() {
            pause_state.unpause();
        } else {
            pause_state.pause();
        }
    }
}