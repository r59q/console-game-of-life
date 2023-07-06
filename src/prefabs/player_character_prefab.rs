use crate::components::controllable::Controllable;
use crate::components::position::Position;
use crate::components::rendering_character::RenderingCharacter;
use crate::components::velocity::Velocity;
use crate::prefabs::{Prefab, Prefabs};
use bevy_ecs::world::EntityMut;

impl Prefabs {
    pub const PLAYER_CHARACTER: Prefab = |mut base_entity: EntityMut| {
        base_entity
            .insert(Position { x: 1., y: 1. })
            .insert(Velocity { x: 0.0, y: 0.0 })
            .insert(Controllable {})
            .insert(RenderingCharacter { character: '@' })
            .id()
    };
}
