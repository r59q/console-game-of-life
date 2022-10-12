use crate::game::ecs::component::Component;

pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn zero() -> Self {
        return Position { x: 0., y: 0. };
    }
}

pub trait Transform: Component {
}

impl Component for Position {}

impl Transform for Position {

}