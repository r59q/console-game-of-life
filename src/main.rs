use bevy_ecs::prelude::SystemStage;

use components::{position::Position, velocity::Velocity};
use game::Game;
use resources::axis_inputs::AxisInputs;
use resources::key_bindings::KeyBindings;
use systems::reset_axis_input::reset_axial_inputs;
use crate::components::rendering_character::RenderingCharacter;

use crate::resources::render_targets::RenderTargets;
use crate::resources::timer::Timer;
use crate::systems::character_renderer::{character_renderer, character_renderer_reset};
use crate::systems::debug_inputs::debug_inputs;
use crate::systems::movement::movement_system;
use crate::systems::timing::timing_system;

mod game;
mod components;
mod systems;
mod resources;
mod inputmanager;

fn main() {
    let mut game: Game = Game::new(3, 3, 30);
    let mut player_entity =
        game.get_world_mut().spawn();
    player_entity
        .insert(Position { x: 0., y: 0. })
        .insert(Velocity { x: 3., y: 1. })
        .insert(RenderingCharacter { character:'@' });

    game.get_world_mut().insert_resource(Timer::new());
    game.get_world_mut().insert_resource(RenderTargets::new());
    game.get_world_mut().insert_resource(AxisInputs::new());
    game.get_world_mut().insert_resource(KeyBindings::new());

    game.add_stage_to_schedule("timing", SystemStage::parallel()
        .with_system(timing_system),
    );
    game.add_stage_to_schedule("update", SystemStage::parallel()
        .with_system(movement_system)
        .with_system(character_renderer_reset)
        .with_system(debug_inputs)
    );
    game.add_stage_to_schedule("pre-render", SystemStage::single_threaded()
        .with_system(character_renderer),
    );
    game.add_stage_to_schedule("post-render", SystemStage::single_threaded()
        .with_system(reset_axial_inputs),
    );
    game.start();
}

#[cfg(test)]
mod tests;