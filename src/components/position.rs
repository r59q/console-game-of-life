use bevy_ecs::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: f64, pub y:f64
}

impl Position {
    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
}