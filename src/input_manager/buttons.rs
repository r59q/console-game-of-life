use strum_macros::EnumIter;

#[derive(Clone, Copy, Eq, Hash, PartialEq, EnumIter)]
pub enum Button {
    Fire1,
    Fire2,
    Buy,
    Place,
    Pause
}