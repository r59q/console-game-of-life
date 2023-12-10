use bevy_ecs::prelude::Component;
use console_engine_TC_FIX::Color;

#[derive(Component, Clone, Copy)]
pub struct RenderingCharacter {
    pub character: char,
    pub color: Color,
}
