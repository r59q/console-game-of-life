use bevy_ecs::prelude::{Query, ResMut};
use crate::components::position::Position;
use crate::components::rendering_character::RenderingCharacter;
use crate::resources::render_targets::{RenderTarget, RenderTargets};

pub fn character_renderer(mut render_targets: ResMut<RenderTargets>, query: Query<(&Position, &RenderingCharacter)>) {
    for (pos, char) in &query {
        let position = pos.clone();
        let character = char.character;
        render_targets.add(RenderTarget::new(position, character));
    }
}

pub fn character_renderer_reset(mut render_targets: ResMut<RenderTargets>) {
    render_targets.reset();
}