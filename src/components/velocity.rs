use bevy_ecs::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct Velocity {
    pub x: f64, pub y:f64
} 