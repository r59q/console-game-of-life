use console_engine::{KeyCode, MouseButton};

#[derive(Clone, Copy)]
pub enum InputType {
    Key(KeyCode),
    Mouse(MouseButton)
}