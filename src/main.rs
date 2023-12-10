use bevy_ecs::prelude::SystemStage;
use console_engine_TC_FIX::KeyCode;
use console_engine_TC_FIX::MouseButton::{Left, Right};
use resources::simulation_speed::SimulationSpeed;
use resources::ui::help_menu_state::HelpMenuState;
use resources::ui::tutorial_preset::insert_tutorial_text;
use resources::ui::ui_layer::UILayer;
use systems::cell_randomizer::cell_randomizer;
use systems::conways_rules::conways_rules;
use systems::positioned_entities_updater::positioned_entities_updater;
use systems::simulation_speed_changer::simulation_speed_changer;
use systems::toggle_cell_on_click::toggle_cell_on_click;
use systems::toggle_help_menu::help_menu_toggeling;

use crate::input_manager::axis::Axis::{Horizontal, Vertical};
use crate::input_manager::buttons::Button::{
    Buy, Decrease, Fire1, Fire2, Help, Increase, Pause, Place, Random,
};
use crate::input_manager::input_types::InputType::{Key, Mouse};

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
    let args = std::env::args().collect::<Vec<String>>();

    // Default fps is 60, check for overriding config from argument
    let mut fps = 60;
    if args.len() > 1 {
        fps = args[1]
            .parse()
            .expect("Wrong argument. Please only put a number to specify target fps")
    }

    let mut game: Game = Game::new(3, 3, fps);

    // game.spawn_prefab(Prefabs::PLAYER_CHARACTER);
    // game.spawn_prefab(Prefabs::PLACEABLE_CELL);

    add_resources(&mut game);

    stage_systems(&mut game);

    game.start();
}

fn add_resources(game: &mut Game) {
    let view_offset = ViewOffset::new();
    let bindings = bind_keys();

    game.get_world_mut().insert_resource(Timer::new());
    game.get_world_mut().insert_resource(RenderTargets::new());
    game.get_world_mut().insert_resource(PauseState::new());
    game.get_world_mut().insert_resource(view_offset);
    game.get_world_mut().insert_resource(bindings);
    game.get_world_mut().insert_resource(HelpMenuState::new());
    game.get_world_mut().insert_resource(AxisInputs::new());
    game.get_world_mut().insert_resource(create_ui_layer());
    game.get_world_mut().insert_resource(MouseInputs::new());
    game.get_world_mut().insert_resource(ButtonInputs::new());
    game.get_world_mut()
        .insert_resource(SimulationSpeed::new(500));
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
    bindings.bind_to_button(Random, Key(KeyCode::Char('r')));
    bindings.bind_to_button(Decrease, Key(KeyCode::Char('z')));
    bindings.bind_to_button(Increase, Key(KeyCode::Char('x')));
    bindings.bind_to_button(Help, Key(KeyCode::Char('h')));

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
            .with_system(help_menu_toggeling)
            .with_system(drag_view_offset)
            .with_system(simulation_speed_changer)
            .with_system(cell_randomizer)
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
    insert_tutorial_text(&mut ui_layer);
    ui_layer
}

#[cfg(test)]
mod tests;
