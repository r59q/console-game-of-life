use bevy_ecs::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: f64, pub y:f64
}