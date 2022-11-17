use bevy_ecs::prelude::SystemStage;
use console_engine::KeyCode;
use console_engine::MouseButton::{Left, Right};

use game::Game;
use resources::inputs::axis_inputs::AxisInputs;
use resources::inputs::input_bindings::InputBindings;
use resources::pause_state::PauseState;
use resources::view_offset::ViewOffset;
use systems::drag_view_offset::drag_view_offset;
use systems::reset_axis_input::reset_axial_inputs;
use crate::input_manager::axis::Axis::{Horizontal, Vertical};
use crate::input_manager::buttons::Button::{Buy, Fire1, Fire2};
use crate::input_manager::input_types::InputType::{Key, Mouse};
use resources::inputs::button_inputs::ButtonInputs;
use resources::inputs::mouse_inputs::MouseInputs;
use crate::prefabs::Prefabs;

use crate::resources::render_targets::RenderTargets;
use crate::resources::timer::Timer;
use crate::systems::axis_position_transform::axis_position_transform;
use crate::systems::axis_velocity::axis_velocity;
use crate::systems::character_renderer::{character_renderer, character_renderer_reset};
use crate::systems::debugger::debugger;
use crate::systems::movement::movement_system;
use crate::systems::reset_mouse_input::reset_mouse_inputs;
use crate::systems::timing::timing_system;

mod game;
mod components;
mod systems;
mod resources;
mod input_manager;
mod prefabs;

fn main() {
    let mut game: Game = Game::new(3, 3, 30);

    game.spawn_prefab(Prefabs::PLAYER_CHARACTER);

    add_resources(&mut game);

    stage_systems(&mut game);

    game.start();
}

fn add_resources(game: &mut Game) {
    game.get_world_mut().insert_resource(Timer::new());
    game.get_world_mut().insert_resource(RenderTargets::new());
    game.get_world_mut().insert_resource(PauseState::new());
    let mut view_offset = ViewOffset::new();
    view_offset.set_offset(10, 5);
    game.get_world_mut().insert_resource(view_offset);

    let bindings = bind_keys();
    game.get_world_mut().insert_resource(bindings);

    game.get_world_mut().insert_resource(AxisInputs::new());
    game.get_world_mut().insert_resource(MouseInputs::new());
    game.get_world_mut().insert_resource(ButtonInputs::new());
}

fn bind_keys() -> InputBindings {
    let mut bindings = InputBindings::new();
    bindings.bind_to_button(Fire1, Mouse(Left));
    bindings.bind_to_button(Fire2, Mouse(Right));
    bindings.bind_to_button(Buy, Key(KeyCode::Char('b')));

    bindings.bind_to_axis(
        Horizontal,
        Key(KeyCode::Char('d')),
        Key(KeyCode::Char('a')),
    );
    bindings.bind_to_axis(
        Vertical,
        Key(KeyCode::Char('s')),
        Key(KeyCode::Char('w')),
    );

    bindings
}

fn stage_systems(game: &mut Game) {
    game.add_stage_to_schedule("timing", SystemStage::parallel()
        .with_system(timing_system),
    );
    game.add_stage_to_schedule("update", SystemStage::parallel()
        .with_system(movement_system)
        .with_system(axis_position_transform)
        .with_system(character_renderer_reset)
        .with_system(drag_view_offset)
        .with_system(debugger),
    );
    game.add_stage_to_schedule("pre-render", SystemStage::single_threaded()
        .with_system(character_renderer),
    );
    game.add_stage_to_schedule("post-render", SystemStage::single_threaded()
        .with_system(reset_axial_inputs)
        .with_system(reset_mouse_inputs),
    );
}

#[cfg(test)]
mod tests;
