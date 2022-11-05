use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumIter)]
pub enum InputAction {
    None,
    Down,
    Held,
    Drag,
    Up,
}