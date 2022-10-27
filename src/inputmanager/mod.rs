mod axis;
mod bindings;

#[cfg(test)]
mod test {
    use console_engine::KeyCode;

    use super::axis::Axis;
    use super::bindings::Bindings;

    #[test]
    fn has_axis_horizontal_vertical() {
        let _horizontal = Axis::Horizontal;
        let _vertical = Axis::Vertical;
    }

    #[test]
    fn has_key_bindings() {
        let _keybindings = Bindings::new();
    }

    #[test]
    fn has_key_horizontal_bindings() {
        let mut keybindings = Bindings::new();
        let _horizontal = keybindings.get_axial_bindings(Axis::Horizontal);
    }

    #[test]
    fn has_bind_axial_key_method() {
        let mut keybindings = Bindings::new();
        keybindings.bind_key_to_axis(Axis::Horizontal, KeyCode::Char('d'),KeyCode::Char('a'));
    }

    #[test]
    fn can_add_new_key_binding() {
        let mut keybindings = Bindings::new();
        keybindings.bind_key_to_axis(Axis::Horizontal, KeyCode::Char('d'),KeyCode::Char('a'));
        assert_eq!(1, keybindings.get_axial_bindings(Axis::Horizontal).len());
    }

    #[test]
    fn can_add_new_correct_binding() {
        let mut keybindings = Bindings::new();
        keybindings.bind_key_to_axis(Axis::Horizontal, KeyCode::Char('d'),KeyCode::Char('a'));
        let binding = keybindings.get_axial_bindings(Axis::Horizontal);
        let opt_binding = binding.get(0);

        match opt_binding {
            Some(binding) => {
                assert_eq!(binding.positive, KeyCode::Char('d'));
                assert_eq!(binding.negative, KeyCode::Char('a'))
            },
            None => assert!(false),
        }
    }
}