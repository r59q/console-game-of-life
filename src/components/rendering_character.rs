use bevy_ecs::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct RenderingCharacter {
    pub character: char
}