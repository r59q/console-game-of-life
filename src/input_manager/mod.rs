pub mod axis;
pub mod buttons;
pub mod input_action;
pub mod input_types;
pub mod key_binding;
pub trait SharedInputBehaviour {
    fn reset_inputs(&mut self);
}
