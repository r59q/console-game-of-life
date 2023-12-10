use console_engine_TC_FIX::{KeyCode, MouseButton};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum InputType {
    Key(KeyCode),
    Mouse(MouseButton),
}
