use std::convert::TryFrom;

#[derive(Hash, Eq, PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical
}

impl TryFrom<i32> for Axis {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Axis::Horizontal as i32 => Ok(Axis::Horizontal),
            x if x == Axis::Vertical as i32 => Ok(Axis::Vertical),
            _ => Err(()),
        }
    }
}