use crate::components::position::Position;
use bevy_ecs::prelude::*;

use crate::components::rendering_character::RenderingCharacter;
use crate::resources::timer::Timer;
use crate::systems::timing::timing_system;

use super::*;

mod axis_transform_position_system_tests;
mod conways_rules_tests;
mod ecs_integration_tests;
mod game_tests;
mod movement_system_tests;
mod place_under_mouse_tests;
mod rendering_tests;
mod resources_tests;
mod spawn_cell_on_click_test;
mod spawn_placeable_entities_tests;

pub struct TestEnv {
    pub game: Game,
    pub entity: Entity,
}

pub fn initialize() -> TestEnv {
    // Not tied to game.
    let entity = World::new().spawn().id();
    let test_env = TestEnv {
        game: Game::new(1, 1, 1),
        entity,
    };
    return test_env;
}

pub fn initialize_game_paused() -> TestEnv {
    // Not tied to game.
    let entity = World::new().spawn().id();
    let mut test_env = TestEnv {
        game: Game::new(1, 1, 1),
        entity,
    };
    let mut pause_state = PauseState::new();
    pause_state.pause();
    test_env.game.get_world_mut().insert_resource(pause_state);
    return test_env;
}

pub fn initialize_with_entity() -> TestEnv {
    let mut game = Game::new(1, 1, 1);
    let entity = game.get_world_mut().spawn().id();
    return TestEnv { game, entity };
}

pub fn initialize_with_entity_and_timing_system() -> TestEnv {
    let mut game = Game::new(1, 1, 1);
    game.get_world_mut().insert_resource(Timer::new());

    game.get_schedule_mut()
        .add_stage("timing", SystemStage::parallel().with_system(timing_system));

    let entity = game.get_world_mut().spawn().id();
    return TestEnv { game, entity };
}

pub fn initialize_with_rendered_entity_and_timing_system() -> TestEnv {
    let mut game = Game::new(1, 1, 1);
    game.get_world_mut().insert_resource(Timer::new());
    game.get_world_mut().insert_resource(RenderTargets::new());

    game.get_schedule_mut()
        .add_stage("timing", SystemStage::parallel().with_system(timing_system));

    game.add_stage_to_schedule(
        "update",
        SystemStage::parallel().with_system(character_renderer_reset),
    );

    game.add_stage_to_schedule(
        "pre-render",
        SystemStage::single_threaded().with_system(character_renderer),
    );

    let entity = game
        .get_world_mut()
        .spawn()
        .insert(Position { x: 0., y: 0. })
        .insert(RenderingCharacter {
            character: 't',
            color: console_engine::Color::Cyan,
        })
        .id();
    return TestEnv { game, entity };
}
