use bevy_ecs::component::Component;

use crate::prefabs::{PrefabCommands};

#[derive(Component, Clone, Copy)]
pub struct Placeable {
    pub replacement: Option<PrefabCommands>
}