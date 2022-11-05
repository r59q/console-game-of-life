use std::collections::HashMap;

use crate::input_manager::{axis::Axis};
use crate::input_manager::buttons::Button;
use crate::input_manager::input_types::InputType;
use crate::input_manager::key_binding::axial_binding::AxialBinding;
use crate::input_manager::key_binding::button_binding::ButtonBinding;

pub struct InputBindings {
    axials: HashMap<Axis, Vec<AxialBinding>>,
    buttons: HashMap<Button, Vec<ButtonBinding>>
}

impl InputBindings {
    pub fn new() -> Self {
        Self { axials: HashMap::new(), buttons: HashMap::new() }
    }

    pub fn get_axial_bindings(&self, axis: Axis) -> Option<&Vec<AxialBinding>> {
        self.axials.get(&axis)
    }

    pub fn bind_to_axis(&mut self, axis: Axis, positive_key: InputType, negative_key: InputType) {
        let bindings = self.axials.entry(axis)
            .or_insert(Vec::<AxialBinding>::new());

        if positive_key == negative_key {
            panic!("Positive key and negative key must differ!")
        }

        for binding in bindings.clone() {
            if binding.negative == negative_key {
                if binding.positive == positive_key {
                    return;
                }
            }
        }

        bindings.push(AxialBinding {positive: positive_key, negative: negative_key})
    }

    pub fn get_button_bindings(&self, button: Button) -> Option<&Vec<ButtonBinding>> {
        self.buttons.get(&button)
    }

    pub fn bind_to_button(&mut self, button: Button, input: InputType) {
        let bindings = self.buttons.entry(button)
            .or_insert(Vec::<ButtonBinding>::new());
        for binding in bindings.clone() {
            if binding.button_input == input {
                return;
            }
        }
        bindings.push(ButtonBinding {button_input: input})
    }
}


#[cfg(test)]
mod test {
    use console_engine::KeyCode;
    use console_engine::MouseButton::{Left, Right};

    use crate::input_manager::axis::Axis::{Horizontal, Vertical};
    use crate::input_manager::buttons::Button::Fire1;
    use crate::input_manager::input_types::InputType::{Key, Mouse};

    use super::InputBindings;


    #[test]
    fn has_axis_horizontal_vertical() {
        let _horizontal = Horizontal;
        let _vertical = Vertical;
    }

    #[test]
    fn has_key_bindings() {
        let _keybindings = InputBindings::new();
    }

    #[test]
    fn has_key_horizontal_bindings() {
        let keybindings = InputBindings::new();
        let _horizontal = keybindings.get_axial_bindings(Horizontal);
    }

    #[test]
    fn has_bind_axial_key_method() {
        let mut keybindings = InputBindings::new();
        keybindings.bind_to_axis(Horizontal, Key(KeyCode::Char('d')), Key(KeyCode::Char('a')));
    }

    #[test]
    fn can_add_new_key_binding() {
        let mut keybindings = InputBindings::new();
        keybindings.bind_to_axis(Horizontal, Key(KeyCode::Char('d')), Key(KeyCode::Char('a')));
        assert_eq!(1, keybindings.get_axial_bindings(Horizontal).unwrap().len());
    }

    #[test]
    fn can_add_new_correct_binding() {
        let mut keybindings = InputBindings::new();

        keybindings.bind_to_axis(
            Horizontal,
            Key(KeyCode::Char('d')),
            Key(KeyCode::Char('a'))
        );
        keybindings.bind_to_axis(
            Vertical,
            Key(KeyCode::Char('w')),
            Key(KeyCode::Char('s'))
        );
        let binding_horizontal = keybindings.get_axial_bindings(Horizontal);
        let binding_vertical = keybindings.get_axial_bindings(Vertical);
        let opt_binding_horizontal = binding_horizontal.unwrap().get(0);
        let opt_binding_vertical = binding_vertical.unwrap().get(0);

        match opt_binding_horizontal {
            Some(binding) => {
                let pos = binding.positive;
                let neg = binding.negative;
                assert!(matches!(pos, Key(KeyCode::Char('d'))));
                assert!(matches!(neg, Key(KeyCode::Char('a'))));
            },
            None => assert!(false),
        }

        match opt_binding_vertical {
            Some(binding) => {
                let pos = binding.positive;
                let neg = binding.negative;
                assert!(matches!(pos, Key(KeyCode::Char('w'))));
                assert!(matches!(neg, Key(KeyCode::Char('s'))));
            },
            None => assert!(false),
        }
    }

    #[test]
    fn can_get_button_binding() {
        let keybindings = InputBindings::new();

        let _fire1_bindings = keybindings.get_button_bindings(Fire1);

    }

    #[test]
    fn can_add_button_binding() {
        let mut keybindings = InputBindings::new();
        keybindings.bind_to_button(Fire1, Mouse(Left));

        let added_binding = keybindings.get_button_bindings(Fire1);

        assert_eq!(added_binding.unwrap().len(), 1);
    }

    #[test]
    fn can_add_another_binding_to_same_button() {
        let mut keybindings = InputBindings::new();
        keybindings.bind_to_button(Fire1, Mouse(Left));
        keybindings.bind_to_button(Fire1, Mouse(Right));

        let added_binding = keybindings.get_button_bindings(Fire1);

        assert_eq!(added_binding.unwrap().len(), 2);
    }

    #[test]
    fn cannot_add_same_binding_twice_and_get_more_bindings() {
        let mut keybindings = InputBindings::new();
        keybindings.bind_to_button(Fire1, Mouse(Left));
        keybindings.bind_to_button(Fire1, Mouse(Left));

        keybindings.bind_to_axis(Horizontal,
                                 Key(KeyCode::Char('x')),
                                 Key(KeyCode::Char('y')));
        keybindings.bind_to_axis(Horizontal,
                                 Key(KeyCode::Char('x')),
                                 Key(KeyCode::Char('y')));

        let added_button_bindings = keybindings.get_button_bindings(Fire1);
        let added_axis_bindings = keybindings.get_axial_bindings(Horizontal);

        assert_eq!(added_button_bindings.unwrap().len(), 1);
        assert_eq!(added_axis_bindings.unwrap().len(), 1);
    }
}