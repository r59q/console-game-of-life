pub mod axis_inputs;
pub mod button_inputs;
pub mod mouse_inputs;
pub mod input_bindings;
pub trait SharedInputBehaviour {
    fn reset_inputs(&mut self);
}
