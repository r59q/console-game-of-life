use crate::components::placeable::Placeable;
use crate::components::position::Position;
use crate::components::rendering_character::RenderingCharacter;
use crate::components::ui_entity_flag::UIEntityFlag;
use crate::prefabs::{Prefab, Prefabs};
use bevy_ecs::prelude::Entity;
use bevy_ecs::system::EntityCommands;
use bevy_ecs::world::EntityMut;

use super::PrefabCommands;

impl Prefabs {
    pub const CELL: PrefabCommands = |mut base_entity: EntityCommands| {
        base_entity
            .insert(Position { x: 0., y: 0. })
            .insert(RenderingCharacter {
                character: '■',
                color: console_engine::Color::Cyan,
            })
            .id()
    };

    pub const CELL_WITHOUT_POSITION: fn(&mut EntityCommands) -> Entity =
        |base_entity: &mut EntityCommands| {
            base_entity
                .insert(RenderingCharacter {
                    character: '■',
                    color: console_engine::Color::Cyan,
                })
                .id()
        };

    pub const PLACEABLE_CELL: Prefab = |mut base_entity: EntityMut| {
        base_entity
            .insert(Position { x: 0., y: 0. })
            .insert(Placeable {
                replacement: Some(Prefabs::CELL),
            })
            .insert(UIEntityFlag {})
            .insert(RenderingCharacter {
                character: '#',
                color: console_engine::Color::Green,
            })
            .id()
    };
}
