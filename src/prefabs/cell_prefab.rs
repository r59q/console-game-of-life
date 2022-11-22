use bevy_ecs::system::EntityCommands;
use bevy_ecs::world::EntityMut;
use crate::components::controllable::Controllable;
use crate::components::placeable::Placeable;
use crate::components::position::Position;
use crate::components::rendering_character::RenderingCharacter;
use crate::components::velocity::Velocity;
use crate::prefabs::{Prefab, Prefabs};

use super::PrefabCommands;

impl Prefabs {
    pub const CELL: PrefabCommands = |mut base_entity: EntityCommands| base_entity
        .insert(Position { x: 0., y: 0. })
        .insert(RenderingCharacter { character:'#' }).id();
    
        pub const PLACEABLE_CELL: Prefab = |mut base_entity: EntityMut| base_entity
        .insert(Position { x: 0., y: 0. })
        .insert(Placeable{ replacement: Some(Prefabs::CELL)})
        .insert(RenderingCharacter { character:'#' }).id();
}
