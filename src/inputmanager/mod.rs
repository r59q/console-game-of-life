use crate::game::Game;

pub mod axis;
pub mod axial_binding;
pub mod capture_inputs;
pub trait Input {
    fn reset_inputs(&mut self);
}
