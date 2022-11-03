use console_engine::{KeyCode, MouseButton};

pub enum InputType {
    Key(KeyCode),
    Mouse(MouseButton)
}