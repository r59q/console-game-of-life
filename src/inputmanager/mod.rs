mod axis;
mod bindings;

#[cfg(test)]
mod test {
    use super::axis::Axis;
    use super::bindings::Bindings;

    #[test]
    fn has_axis_horizontal_vertical() {
        let _horizontal = Axis::Horizontal;
        let _vertical = Axis::Vertical;
    }

    #[test]
    fn has_key_bindings_for_axis() {
        let keybindings = Bindings::axials;
    }
}