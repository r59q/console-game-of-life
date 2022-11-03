use std::collections::HashMap;

use console_engine::MouseButton;

#[derive(Clone, Copy, PartialEq, Eq)]
enum MouseAction {
    None,
    Down,
}


pub struct MouseState {
    action: MouseAction
}

impl MouseState {
    fn new() -> Self {
        return MouseState { action: MouseAction::None  };
    }

    fn get_action(&self) -> MouseAction {
        return self.action;
    }

    fn set_action(&mut self, action: MouseAction) {
        self.action = action;
    }
}

pub struct MouseInputs {
    inputs: HashMap<MouseButton, MouseState>
}
impl MouseInputs {
    pub(crate) fn new() -> MouseInputs {
        return MouseInputs { inputs: HashMap::new() }
    }

    pub(crate) fn get_state(&mut self, mouse_button: MouseButton) -> &MouseState {
        return self.inputs.entry(mouse_button).or_insert(MouseState::new());
    }

    pub(crate) fn set_state(&self, button: MouseButton, x: u32, y: u32) {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use console_engine::MouseButton;

    use super::{MouseInputs, MouseState, MouseAction};

    #[test]
    fn can_create_mouse_inputs() {
        let mouse_inputs: MouseInputs = MouseInputs::new();
    }

    #[test]
    fn can_get_left_mouse_state() {
        let left_button = MouseButton::Left;
        let left_state: &MouseState = 
            MouseInputs::new().get_state(left_button);
    }

    #[test]
    fn action_starts_as_none() {
        let left_button = MouseButton::Left;
        let mut binding = MouseInputs::new();
        let left_state: &MouseState = 
            binding.get_state(left_button);

        let action: MouseAction = left_state.get_action();
        assert!(matches!(action, MouseAction::None));
    }

    #[test]
    fn can_change_action() {
        let left_button = MouseButton::Left;
        let mut binding = MouseInputs::new();
        let left_state: &MouseState = 
            binding.get_state(left_button);

            binding.set_state(left_button, 2, 3);

        let action: MouseAction = left_state.get_action();
        assert!(matches!(action, MouseAction::None));
    }
}