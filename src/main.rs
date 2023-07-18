use bevy_ecs::prelude::SystemStage;
use console_engine::MouseButton::{Left, Right};
use console_engine::{Color, KeyCode};
use resources::ui::ui_layer::UILayer;
use systems::conways_rules::conways_rules;
use systems::positioned_entities_updater::positioned_entities_updater;
use systems::toggle_cell_on_click::toggle_cell_on_click;

use crate::input_manager::axis::Axis::{Horizontal, Vertical};
use crate::input_manager::buttons::Button::{Buy, Fire1, Fire2, Pause, Place};
use crate::input_manager::input_types::InputType::{Key, Mouse};
use crate::prefabs::Prefabs;
use game::Game;
use resources::inputs::axis_inputs::AxisInputs;
use resources::inputs::button_inputs::ButtonInputs;
use resources::inputs::input_bindings::InputBindings;
use resources::inputs::mouse_inputs::MouseInputs;
use resources::pause_state::PauseState;
use resources::positioned_entities::PositionedEntities;
use resources::view_offset::ViewOffset;
use systems::drag_view_offset::drag_view_offset;
use systems::pause_toggle::pause_toggle;
use systems::place_under_mouse::place_under_mouse;
use systems::reset_axis_input::reset_axial_inputs;
use systems::spawn_placeables::spawn_placeables;

use crate::resources::render_targets::RenderTargets;
use crate::resources::timer::Timer;
use crate::systems::axis_position_transform::axis_position_transform;
use crate::systems::character_renderer::{character_renderer, character_renderer_reset};
use crate::systems::debugger::debugger;
use crate::systems::movement::movement_system;
use crate::systems::reset_mouse_input::reset_mouse_inputs;
use crate::systems::timing::timing_system;

mod components;
mod game;
mod input_manager;
mod prefabs;
mod resources;
mod systems;

fn main() {
    let mut game: Game = Game::new(3, 3, 30);

    // game.spawn_prefab(Prefabs::PLAYER_CHARACTER);
    // game.spawn_prefab(Prefabs::PLACEABLE_CELL);

    add_resources(&mut game);

    stage_systems(&mut game);

    game.start();
}

fn add_resources(game: &mut Game) {
    game.get_world_mut().insert_resource(Timer::new());
    game.get_world_mut().insert_resource(RenderTargets::new());
    game.get_world_mut().insert_resource(PauseState::new());
    let mut view_offset = ViewOffset::new();
    view_offset.set_offset(0, 0);
    game.get_world_mut().insert_resource(view_offset);

    let bindings = bind_keys();
    game.get_world_mut().insert_resource(bindings);

    game.get_world_mut().insert_resource(AxisInputs::new());
    game.get_world_mut().insert_resource(create_ui_layer());
    game.get_world_mut().insert_resource(MouseInputs::new());
    game.get_world_mut().insert_resource(ButtonInputs::new());
    game.get_world_mut()
        .insert_resource(PositionedEntities::new());
}

fn bind_keys() -> InputBindings {
    let mut bindings = InputBindings::new();
    bindings.bind_to_button(Fire1, Mouse(Left));
    bindings.bind_to_button(Fire2, Mouse(Right));
    bindings.bind_to_button(Buy, Key(KeyCode::Char('b')));
    bindings.bind_to_button(Pause, Key(KeyCode::Char('p')));
    bindings.bind_to_button(Place, Key(KeyCode::Char('q')));

    bindings.bind_to_axis(Horizontal, Key(KeyCode::Char('d')), Key(KeyCode::Char('a')));
    bindings.bind_to_axis(Vertical, Key(KeyCode::Char('s')), Key(KeyCode::Char('w')));

    bindings
}

fn stage_systems(game: &mut Game) {
    game.add_stage_to_schedule("timing", SystemStage::parallel().with_system(timing_system));
    game.add_stage_to_schedule(
        "update",
        SystemStage::parallel()
            .with_system(movement_system)
            .with_system(axis_position_transform)
            .with_system(character_renderer_reset)
            .with_system(pause_toggle)
            .with_system(place_under_mouse)
            .with_system(spawn_placeables)
            .with_system(positioned_entities_updater)
            .with_system(drag_view_offset)
            //.with_system(debugger)
            .with_system(toggle_cell_on_click),
    );
    game.add_stage_to_schedule(
        "post-update",
        SystemStage::single_threaded().with_system(conways_rules),
    );
    game.add_stage_to_schedule(
        "render",
        SystemStage::single_threaded().with_system(character_renderer),
    );
    game.add_stage_to_schedule(
        "post-render",
        SystemStage::single_threaded()
            .with_system(reset_axial_inputs)
            .with_system(reset_mouse_inputs),
    );
}

fn create_ui_layer() -> UILayer {
    let mut ui_layer = UILayer::new();
    ui_layer.insert_text(
        "*********************************************",
        (1, 1),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "*           Conway's Game of Life           *",
        (1, 2),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "*            --   Help menu   --            *",
        (1, 3),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "*                                           *",
        (1, 4),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "* ESC              - Closes program         *",
        (1, 5),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "* P                - Pause / Unpause        *",
        (1, 6),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "* Left click       - Place cell             *",
        (1, 7),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "* Right click+drag - Drags simulation space *",
        (1, 8),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "* H                - Toggle help menu       *",
        (1, 9),
        Color::Yellow,
    );
    ui_layer.set_pixel((3, 5), 'E', Color::Red);
    ui_layer.set_pixel((4, 5), 'S', Color::Red);
    ui_layer.set_pixel((5, 5), 'C', Color::Red);
    ui_layer.set_pixel((3, 6), 'P', Color::Red);
    ui_layer.insert_text("Left click", (3, 7), Color::Red);
    ui_layer.insert_text("Right click+drag", (3, 8), Color::Red);
    ui_layer.set_pixel((3, 9), 'H', Color::Red);
    ui_layer
}

#[cfg(test)]
mod tests;
