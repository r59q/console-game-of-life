pub struct AxisInputs {
}

impl AxisInputs {
    pub fn new() -> Self {
        return Self { };
    }

    fn get(&self, horizontal: crate::inputmanager::axis::Axis) -> Option<f64> {
        return None;
    }
}

#[cfg(test)]
mod test {

    use super::AxisInputs;
    use crate::inputmanager::axis::Axis;

    #[test]
    fn can_get_axial_input() {
        let axis_inputs = AxisInputs::new();

        let horizontal_input_value = axis_inputs.get(Axis::Horizontal);
        assert!(matches!(horizontal_input_value, None))
    }
}