use std::collections::HashMap;

use bevy_ecs::schedule::StateError;
use console_engine::MouseButton;

use crate::inputmanager::Input;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MouseAction {
    None,
    Down,
}


#[derive(Debug)]
pub struct MouseState {
    action: MouseAction
}
// todo: Mouse state may be too much. Might be replaced with just the action.
impl MouseState {
    fn new() -> Self {
        return MouseState { action: MouseAction::None  };
    }

    pub fn get_action(&self) -> MouseAction {
        return self.action;
    }

    fn set_action(&mut self, action: MouseAction) {
        self.action = action;
    }
}

#[derive(Debug)]
pub struct MouseInputs {
    inputs: HashMap<MouseButton, MouseState>,
    position: (u32, u32)
}
impl MouseInputs {
    pub(crate) fn new() -> MouseInputs {
        return MouseInputs { inputs: HashMap::new(), position: (0,0) }
    }

    pub(crate) fn get_state(&mut self, mouse_button: MouseButton) -> &MouseState {
        return self.inputs.entry(mouse_button).or_insert(MouseState::new());
    }

    pub(crate) fn set_state(&mut self, button: MouseButton, action: MouseAction, x: u32, y: u32) {
        let state = self.inputs.entry(button).or_insert(MouseState::new());
        state.action = action;
        self.position = (x, y)
    }

    pub(crate) fn get_position(&self) -> (u32, u32) {
        return self.position;
    }
}

impl Input for MouseInputs {
    fn reset_inputs(&mut self) {
        self.inputs = Self::new().inputs;
    }
}

#[cfg(test)]
mod test {
    use console_engine::MouseButton;

    use crate::inputmanager::Input;

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
    fn can_set_action() {
        let left_button = MouseButton::Left;
        let mut binding = MouseInputs::new();

        binding.set_state(left_button, MouseAction::Down, 2, 3);

        let left_state: &MouseState = 
            binding.get_state(left_button);

        let action: MouseAction = left_state.get_action();
        assert!(matches!(action, MouseAction::Down));
        assert_eq!(binding.position, (2, 3))
    }

    #[test]
    fn can_reset_inputs() {
        let left_button = MouseButton::Left;
        let mut binding = MouseInputs::new();

        binding.set_state(left_button, MouseAction::Down, 2, 3);

        let mut left_state: &MouseState = 
            binding.get_state(left_button);

        let mut action: MouseAction = left_state.get_action();
        assert!(matches!(action, MouseAction::Down));
        assert_eq!(binding.get_position(), (2, 3));

        binding.reset_inputs();
        left_state = 
            binding.get_state(left_button);

        action = left_state.get_action();
        assert!(matches!(action, MouseAction::None));
        assert_eq!(binding.get_position(), (2, 3));

    }
}