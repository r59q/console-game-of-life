use bevy_ecs::entity::Entity;
use bevy_ecs::system::EntityCommands;
use bevy_ecs::world::EntityMut;

mod cell_prefab;
pub mod player_character_prefab;

pub struct Prefabs {}

pub type Prefab = fn(EntityMut) -> Entity;
pub type PrefabCommands = fn(EntityCommands) -> Entity;
