use std::collections::HashMap;
use std::iter::Map;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum MouseButton {
    LEFT,
    RIGHT,
    NONE,
}

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

pub struct KeyboardInputs {}

pub struct Inputs {
    mouse_inputs: MouseInputs,
    mouse_positions: HashMap<MouseButton, Option<(u32, u32)>>,
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
            mouse_positions: HashMap::from([
                (MouseButton::LEFT, None),
                (MouseButton::NONE, None),
                (MouseButton::RIGHT, None)
            ]),
            keyboard: KeyboardInputs {},
        };
    }

    pub fn get_mouse_down(&self) -> MouseButton {
        return self.mouse_inputs.down;
    }

    pub fn register_mouse_press(&mut self, button: MouseButton, direction: Direction, position: (u32, u32)) {
        match direction {
            Direction::UP => { self.mouse_inputs.up = button }
            Direction::HELD => { self.mouse_inputs.held = button }
            Direction::DOWN => { self.mouse_inputs.down = button }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::resources::inputs::{Direction, Inputs, MouseButton};

    #[test]
    fn can_get_mouse_down() {
        let inputs = Inputs::new();

        let mouse_down = inputs.get_mouse_down();

        match mouse_down {
            MouseButton::LEFT => { assert!(false) }
            MouseButton::RIGHT => { assert!(false) }
            MouseButton::NONE => { assert!(true) }
        }
    }

    #[test]
    fn can_set_button_down() {
        let mut inputs = Inputs::new();

        inputs.register_mouse_press(MouseButton::LEFT, Direction::DOWN, (0, 0));

        let mouse_down = inputs.get_mouse_down();

        match mouse_down {
            MouseButton::LEFT => { assert!(true) }
            MouseButton::RIGHT => { assert!(false) }
            MouseButton::NONE => { assert!(false) }
        }
    }
}