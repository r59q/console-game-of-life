use crate::inputmanager::{bindings::{AxialBinding, Bindings}, Input};

pub struct AxisInputs {
    bindings: Vec<Bindings>
}

impl AxisInputs {
    pub fn new() -> Self {
        return Self { bindings: Vec::new() };
    }

    fn get(&self, axis: crate::inputmanager::axis::Axis) -> f64 {
        return 0.;
    }

    fn add_binding(&mut self, bindings: crate::inputmanager::bindings::Bindings) {
        self.bindings.push(bindings);
    }

    fn get_bindings(&self, axis: crate::inputmanager::axis::Axis) -> Option<AxialBinding> {
        todo!();
    }
}

impl Input for AxisInputs {
    fn reset_inputs(&self) {
        todo!()
    }
}

#[cfg(test)]
mod test {

    use console_engine::KeyCode;

    use super::AxisInputs;
    use crate::inputmanager::{axis::Axis, bindings::{self, AxialBinding}, self, Input};

    #[test]
    fn can_get_axial_input() {
        let axis_inputs = AxisInputs::new();

        let horizontal_input_value = axis_inputs.get(Axis::Horizontal);
        assert_eq!(horizontal_input_value, 0.);
    }

    #[test]
    fn can_add_bindings_to_axis_inputs() {
        let mut axis_inputs = AxisInputs::new();
        let bindings = bindings::Bindings::new();
        axis_inputs.add_binding(bindings);
        let horizontal_input_value = axis_inputs.get_bindings(Axis::Horizontal);
        assert!(matches!(horizontal_input_value, Some(_)))
    }

    #[test]
    fn adds_correct_key_binding() {
        let mut axis_inputs = AxisInputs::new();
        let mut bindings = bindings::Bindings::new();
        let axial_binding 
            = AxialBinding { positive: KeyCode::Char('x'), negative: KeyCode::Char('y')  };

        bindings.bind_key_to_axis(
            Axis::Horizontal, 
            axial_binding.positive, 
            axial_binding.negative);

        axis_inputs.add_binding(bindings);
        let horizontal_input_value = axis_inputs.get_bindings(Axis::Horizontal);
        assert!(matches!(horizontal_input_value, Some(_)))
    }

    #[test]
    fn can_reset_input() {
        let mut axis_inputs = AxisInputs::new();
        let mut bindings = bindings::Bindings::new();
        let axial_binding 
            = AxialBinding { positive: KeyCode::Char('x'), negative: KeyCode::Char('y')  };

        bindings.bind_key_to_axis(
            Axis::Horizontal, 
            axial_binding.positive, 
            axial_binding.negative);

        axis_inputs.add_binding(bindings);
        
        axis_inputs.reset_inputs();

        let horizontal_input_value = axis_inputs.get(Axis::Horizontal);
        assert_eq!(horizontal_input_value, 0.)
    }
}