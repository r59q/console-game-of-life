use crate::input_manager::input_types::InputType;

#[derive(Clone, Copy)]
pub struct AxialBinding {
    pub positive: InputType,
    pub negative: InputType
}
