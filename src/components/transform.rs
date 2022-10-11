use crate::game::ecs::component::Component;

pub struct Position {
    x: f64,
    y: f64,
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