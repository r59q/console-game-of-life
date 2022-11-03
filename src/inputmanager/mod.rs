use crate::game::Game;

pub mod axis;
pub mod axial_binding;
pub mod capture_inputs;
pub mod input_types;
pub trait SharedInputBehaviour {
    fn reset_inputs(&mut self);
}
