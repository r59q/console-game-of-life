use std::collections::HashMap;
use std::iter::Map;

use console_engine::KeyCode;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum MouseButton {
    LEFT,
    RIGHT,
    NONE,
}

#[derive(Clone, Copy)]
enum Direction {
    UP,
    HELD,
    DOWN,
}

pub struct MouseInputs {
    down: MouseButton,
    held: MouseButton,
    up: MouseButton,
}

pub struct KeyboardInputs {
    down: KeyCode,
    held: KeyCode,
    up: KeyCode,
}

pub struct Inputs {
    mouse_inputs: MouseInputs,
    mouse_positions: Option<(u32, u32)>,
    keyboard: KeyboardInputs,
}

impl Inputs {
    pub fn new() -> Self {
        return Self {
            mouse_inputs: MouseInputs {
                down: MouseButton::NONE,
                held: MouseButton::NONE,
                up: MouseButton::NONE,
            },
            mouse_positions: None,
            keyboard: KeyboardInputs {
                up: KeyCode::Null,
                down: KeyCode::Null,
                held: KeyCode::Null,
            },
        };
    }

    pub fn reset_inputs(&mut self) {
        let reset_obj = Self::new();
        self.mouse_inputs = reset_obj.mouse_inputs;
        self.keyboard = reset_obj.keyboard;
        self.mouse_positions = reset_obj.mouse_positions;
    }

    pub fn get_mouse_down(&self, button: MouseButton) -> bool {
        return button == self.mouse_inputs.down;
    }

    pub fn get_mouse_up(&self, button: MouseButton) -> bool {
        return button == self.mouse_inputs.up;
    }

    pub fn get_mouse_held(&self, button: MouseButton) -> bool {
        return button == self.mouse_inputs.held;
    }

    pub fn get_key_down(&self, key: KeyCode) -> bool {
        return key == self.keyboard.down;
    }

    pub fn get_key_up(&self, key: KeyCode) -> bool {
        return key == self.keyboard.up;
    }

    pub fn get_key_held(&self, key: KeyCode) -> bool {
        return key == self.keyboard.held;
    }

    pub fn get_mouse_position(&self) -> Option<(u32, u32)> {
        return self.mouse_positions;
    }

    pub fn register_mouse_press(&mut self, button: MouseButton, direction: Direction, position: (u32, u32)) {
        match direction {
            Direction::UP => { self.mouse_inputs.up = button }
            Direction::HELD => { self.mouse_inputs.held = button }
            Direction::DOWN => { self.mouse_inputs.down = button }
        }
        self.mouse_positions = Some(position);
    }

    pub fn register_key_press(&mut self, button: KeyCode, direction: Direction) {
        match direction {
            Direction::UP => { self.keyboard.up = button }
            Direction::HELD => { self.keyboard.held = button }
            Direction::DOWN => { self.keyboard.down = button }
        }
    }
}

#[cfg(test)]
mod test {
    use console_engine::KeyCode;

    use Direction::UP;
    use MouseButton::{LEFT, NONE, RIGHT};

    use crate::resources::inputs::{Direction, Inputs, MouseButton};
    use crate::resources::inputs::Direction::{DOWN, HELD};

    #[test]
    fn can_get_mouse_down() {
        let inputs = Inputs::new();

        let mouse_down = inputs.get_mouse_down(LEFT);

        assert_eq!(mouse_down, false)
    }

    #[test]
    fn can_register_mouse_down() {
        let mut inputs = Inputs::new();

        inputs.register_mouse_press(LEFT, DOWN, (0, 0));

        let mouse_down = inputs.get_mouse_down(LEFT);

        assert_eq!(mouse_down, true)
    }

    #[test]
    fn can_register_mouse_up() {
        let mut inputs = Inputs::new();

        inputs.register_mouse_press(LEFT, UP, (0, 0));

        let mouse_down = inputs.get_mouse_up(LEFT);
        assert_eq!(mouse_down, true);
        assert_eq!(inputs.get_mouse_up(RIGHT), false);
        assert_eq!(inputs.get_mouse_up(NONE), false);
    }

    #[test]
    fn can_get_mouse_up_where_none_are() {
        let mut inputs = Inputs::new();

        let mouse_down = inputs.get_mouse_up(NONE);

        assert_eq!(mouse_down, true)
    }

    #[test]
    fn can_none_option_position() {
        let mut inputs = Inputs::new();

        let mouse_pos: Option<(u32, u32)> = inputs.get_mouse_position();
        match mouse_pos {
            None => { assert!(true) }
            Some(_) => { assert!(false) }
        }
    }

    #[test]
    fn can_some_option_position_after_click() {
        let mut inputs = Inputs::new();

        inputs.register_mouse_press(LEFT, DOWN, (0, 0));

        let mouse_pos: Option<(u32, u32)> = inputs.get_mouse_position();
        match mouse_pos {
            None => { assert!(false) }
            Some(_) => { assert!(true) }
        }
    }

    #[test]
    fn can_get_correct_some_option_position_after_click() {
        let mut inputs = Inputs::new();

        inputs.register_mouse_press(LEFT, DOWN, (3311, 1333));

        let mouse_pos: Option<(u32, u32)> = inputs.get_mouse_position();
        match mouse_pos {
            None => { assert!(false) }
            Some((x, y)) => {
                assert_eq!(3311, x);
                assert_eq!(1333, y);
            }
        }
    }

    #[test]
    fn can_reset_inputs() {
        let mut inputs = Inputs::new();

        inputs.register_mouse_press(LEFT, DOWN, (3311, 1333));
        inputs.register_mouse_press(RIGHT, DOWN, (3311, 1333));

        inputs.register_key_press(KeyCode::Char('a'), DOWN);

        inputs.reset_inputs();

        let mouse_pos: Option<(u32, u32)> = inputs.get_mouse_position();
        let mouse_down = inputs.get_mouse_down(LEFT);

        assert_eq!(false, mouse_down);
        match mouse_pos {
            None => { assert!(true) }
            Some(_) => { assert!(false) }
        }
        let key_down = inputs.get_key_down(KeyCode::Char('a'));

        assert_eq!(false, key_down);
    }

    #[test]
    fn can_get_keyboard_down() {
        let mut inputs = Inputs::new();

        let a_down = inputs.get_key_down(KeyCode::Char('a'));

        assert_eq!(false, a_down)
    }

    #[test]
    fn can_register_keyboard_down() {
        let mut inputs = Inputs::new();

        inputs.register_key_press(KeyCode::Char('a'), DOWN);

        let a_down = inputs.get_key_down(KeyCode::Char('a'));

        assert_eq!(true, a_down);
        assert_eq!(false, inputs.get_key_down(KeyCode::Char('b')));
    }

    #[test]
    fn can_register_keyboard_up() {
        let mut inputs = Inputs::new();

        inputs.register_key_press(KeyCode::Char('a'), UP);

        let a_down = inputs.get_key_down(KeyCode::Char('a'));
        let a_up = inputs.get_key_up(KeyCode::Char('a'));

        assert_eq!(true, a_up);
        assert_eq!(false, a_down);
    }

    #[test]
    fn can_register_keyboard_held() {
        let mut inputs = Inputs::new();

        inputs.register_key_press(KeyCode::Char('a'), HELD);

        let a_down = inputs.get_key_down(KeyCode::Char('a'));
        let a_up = inputs.get_key_up(KeyCode::Char('a'));
        let a_held = inputs.get_key_held(KeyCode::Char('a'));

        assert_eq!(false, a_up);
        assert_eq!(false, a_down);
        assert_eq!(true, a_held);
    }

    #[test]
    fn can_register_mouse_held() {
        let mut inputs = Inputs::new();

        inputs.register_mouse_press(LEFT, HELD, (123, 321));

        let get_left = inputs.get_mouse_held(LEFT);
        let mouse_pos = inputs.get_mouse_position();

        assert_eq!(true, get_left);
        assert_eq!((123, 321), mouse_pos.unwrap());
    }
}