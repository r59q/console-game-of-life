use std::borrow::BorrowMut;

use bevy_ecs::system::{Res, ResMut};

use crate::{
    input_manager::buttons::Button,
    resources::{
        inputs::button_inputs::ButtonInputs,
        ui::{help_menu_state::HelpMenuState, tutorial_preset, ui_layer::UILayer},
    },
};

pub fn help_menu_toggeling(
    button_inputs: Res<ButtonInputs>,
    mut help_menu_state: ResMut<HelpMenuState>,
    mut ui_layer: ResMut<UILayer>,
) {
    if button_inputs.get_btn_down(Button::Help) {
        help_menu_state.toggle_visibility();
        if help_menu_state.is_visible() {
            tutorial_preset::insert_tutorial_text(ui_layer.borrow_mut());
        } else {
            tutorial_preset::clear_tutorial_text(ui_layer.borrow_mut());
            tutorial_preset::insert_help_toggle_hint(ui_layer.borrow_mut());
        }
    }
}
