use console_engine::{KeyCode, MouseButton};

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum InputType {
    Key(KeyCode),
    Mouse(MouseButton),
}
