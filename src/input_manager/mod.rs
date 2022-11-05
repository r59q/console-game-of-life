pub mod axis;
pub mod axial_binding;
pub mod button_binding;
pub mod buttons;
pub mod input_action;
pub mod capture_inputs;
pub mod input_types;
pub trait SharedInputBehaviour {
    fn reset_inputs(&mut self);
}
