use std::collections::HashMap;
use crate::input_manager::buttons::Button;
use crate::input_manager::input_action::InputAction;

pub struct ButtonInputs {
    input_map: HashMap<Button, InputAction>,
}

impl ButtonInputs {
    pub(crate) fn new() -> Self {
        ButtonInputs { input_map: HashMap::new() }
    }
    pub(crate) fn get_btn_down(&self, button_name: Button) -> bool {
        let input_action = self.input_map.get(&button_name);
        if let Some(action) = input_action {
            if let InputAction::Down = action {
                return true;
            }
        }
        return false;
    }

    pub(crate) fn set_btn(&mut self, button_name: Button, action: InputAction) {
        self.input_map.entry(button_name)
            .and_modify(|val| *val = action)
            .or_insert(action);
    }

    pub(crate) fn get_btn_action(&self, button_name: Button) -> &InputAction {
        let input = self.input_map.get(&button_name);
        return match input {
            None => &InputAction::None,
            Some(action) => action,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::input_manager::buttons::Button;
    use crate::input_manager::input_action::InputAction;
    use crate::resources::inputs::button_inputs::ButtonInputs;

    use strum::IntoEnumIterator;
    #[test]
    fn can_make_struct() {
        let _inputs = ButtonInputs::new();
    }

    #[test]
    fn can_get_button() {
        let inputs = ButtonInputs::new();

        let fire1 = inputs.get_btn_down(Button::Fire1);
        assert_eq!(fire1, false)
    }

    #[test]
    fn can_set_button() {
        let mut inputs = ButtonInputs::new();

        for action in InputAction::iter() {
            inputs.set_btn(Button::Fire1, action);
            let fire1 = inputs.get_btn_action(Button::Fire1);
            assert_eq!(fire1, &action)
        }
    }
}