pub struct AxisInputs {
}

impl AxisInputs {
    pub fn new() -> Self {
        return Self { };
    }

    fn get(&self, horizontal: crate::inputmanager::axis::Axis) -> Option<f64> {
        return None;
    }

    fn add_binding(&self, bindings: crate::inputmanager::bindings::Bindings) {
        todo!()
    }
}

#[cfg(test)]
mod test {

    use console_engine::KeyCode;

    use super::AxisInputs;
    use crate::inputmanager::{axis::Axis, bindings, self};

    #[test]
    fn can_get_axial_input() {
        let axis_inputs = AxisInputs::new();

        let horizontal_input_value = axis_inputs.get(Axis::Horizontal);
        assert!(matches!(horizontal_input_value, None))
    }

    #[test]
    fn can_add_bindings_to_axis_inputs() {
        let axis_inputs = AxisInputs::new();
        let bindings = bindings::Bindings::new();
        axis_inputs.add_binding(bindings);
        let horizontal_input_value = axis_inputs.get(Axis::Horizontal);
        assert!(matches!(horizontal_input_value, None))
    }
}