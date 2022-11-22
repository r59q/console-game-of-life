use bevy_ecs::world::EntityMut;
use crate::components::controllable::Controllable;
use crate::components::position::Position;
use crate::components::rendering_character::RenderingCharacter;
use crate::components::velocity::Velocity;
use crate::prefabs::{Prefab, Prefabs};

impl Prefabs {
    pub const CELL: Prefab = |mut base_entity: EntityMut| base_entity
        .insert(Position { x: 0., y: 0. })
        .insert(RenderingCharacter { character:'#' }).id();
}
