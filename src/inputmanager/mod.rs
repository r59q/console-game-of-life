pub mod axis;
pub mod axial_binding;

pub trait Input {
    fn reset_inputs(&mut self);
}
