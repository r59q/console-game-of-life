use bevy_ecs::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: f64, pub y:f64
}

#[derive(Component, Clone, Copy)]
pub struct PositionInt {
    pub x: i64, pub y:i64
}