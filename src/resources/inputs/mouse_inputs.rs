use std::collections::HashMap;

use console_engine::MouseButton;
use crate::input_manager::input_action::InputAction;
use crate::resources::inputs::SharedInputBehaviour;


#[derive(Debug)]
pub struct MouseState {
    action: InputAction,
}

impl MouseState {
    fn new() -> Self {
        return MouseState { action: InputAction::None };
    }

    pub fn get_action(&self) -> InputAction {
        return self.action;
    }

    fn set_action(&mut self, action: InputAction) {
        self.action = action;
    }
}

#[derive(Debug)]
pub struct MouseInputs {
    mouse_input_map: HashMap<MouseButton, MouseState>,
    mouse_position: (u32, u32),
    drag_start_pos: (u32, u32)
}

impl MouseInputs {
    pub(crate) fn new() -> MouseInputs {
        return MouseInputs {
            mouse_input_map: HashMap::new(),
            mouse_position: (0, 0),
            drag_start_pos: (0, 0),
        }
    }

    pub(crate) fn get_state(&mut self, mouse_button: MouseButton) -> &MouseState {
        return self.mouse_input_map.entry(mouse_button).or_insert(MouseState::new());
    }

    pub(crate) fn set_state(&mut self, button: MouseButton, action: InputAction, x: u32, y: u32) {
        let state = self.mouse_input_map.entry(button).or_insert(MouseState::new());
        state.set_action(action);
        self.mouse_position = (x, y);
        
        if action == InputAction::Down {
            self.drag_start_pos = (x, y)
        }
        if action == InputAction::Up {
            self.drag_start_pos = (x, y)
        }
    }

    pub(crate) fn get_position(&self) -> (u32, u32) {
        return self.mouse_position;
    }

    pub(crate) fn get_drag_delta(&self) -> (i32, i32) {
        let m_pos_x = self.mouse_position.0 as i32;
        let m_pos_y = self.mouse_position.1 as i32;
        let down_pos_x = self.drag_start_pos.0 as i32;
        let down_pos_y = self.drag_start_pos.1 as i32;
        return (m_pos_x - down_pos_x, m_pos_y - down_pos_y);
    }

}

impl SharedInputBehaviour for MouseInputs {
    fn reset_inputs(&mut self) {
        self.mouse_input_map = Self::new().mouse_input_map;
    }
}

#[cfg(test)]
mod test {
    use console_engine::MouseButton;
    use crate::resources::inputs::SharedInputBehaviour;


    use super::{MouseInputs, MouseState, InputAction};

    #[test]
    fn can_create_mouse_inputs() {
        let _mouse_inputs: MouseInputs = MouseInputs::new();
    }

    #[test]
    fn can_get_left_mouse_state() {
        let left_button = MouseButton::Left;
        let _left_state: &MouseState =
            MouseInputs::new().get_state(left_button);
    }

    #[test]
    fn action_starts_as_none() {
        let left_button = MouseButton::Left;
        let mut binding = MouseInputs::new();
        let left_state: &MouseState = 
            binding.get_state(left_button);

        let action: InputAction = left_state.get_action();
        assert!(matches!(action, InputAction::None));
    }

    #[test]
    fn can_set_action() {
        let left_button = MouseButton::Left;
        let mut binding = MouseInputs::new();

        binding.set_state(left_button, InputAction::Down, 2, 3);

        let left_state: &MouseState = 
            binding.get_state(left_button);

        let action: InputAction = left_state.get_action();
        assert!(matches!(action, InputAction::Down));
        assert_eq!(binding.mouse_position, (2, 3))
    }

    #[test]
    fn can_reset_inputs() {
        let left_button = MouseButton::Left;
        let mut binding = MouseInputs::new();

        binding.set_state(left_button, InputAction::Down, 2, 3);

        let mut left_state: &MouseState = 
            binding.get_state(left_button);

        let mut action: InputAction = left_state.get_action();
        assert!(matches!(action, InputAction::Down));
        assert_eq!(binding.get_position(), (2, 3));

        binding.reset_inputs();
        left_state = 
            binding.get_state(left_button);

        action = left_state.get_action();
        assert!(matches!(action, InputAction::None));
        assert_eq!(binding.get_position(), (2, 3));

    }

    #[test]
    fn can_get_delta() {
        let left_button = MouseButton::Left;
        let mut binding = MouseInputs::new();

        binding.set_state(left_button, InputAction::Down, 2, 3);

        let delta: (i32, i32) = binding.get_drag_delta();

        assert_eq!((0,0), delta);
    }

    #[test]
    fn delta_changes() {
        let left_button = MouseButton::Left;
        let mut binding = MouseInputs::new();

        binding.set_state(left_button, InputAction::Down, 2, 3);
        binding.set_state(left_button, InputAction::Drag, 3, 4);

        let delta: (i32, i32) = binding.get_drag_delta();

        assert_eq!((1,1), delta);
    }

    #[test]
    fn delta_resets() {
        let left_button = MouseButton::Left;
        let mut binding = MouseInputs::new();

        binding.set_state(left_button, InputAction::Down, 2, 3);
        binding.set_state(left_button, InputAction::Drag, 3, 4);
        binding.set_state(left_button, InputAction::Up, 3, 4);

        let delta: (i32, i32) = binding.get_drag_delta();

        assert_eq!((0,0), delta);
    }
}