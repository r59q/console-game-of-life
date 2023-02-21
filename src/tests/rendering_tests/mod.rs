mod view_offset_tests;
mod render_targets_tests;

use crate::components::rendering_character::RenderingCharacter;
use crate::systems::character_renderer::character_renderer;

use super::*;

#[test]
fn can_add_rendering_character_component() {
    let mut test_env = initialize();

    test_env.game.get_world_mut().spawn()
        .insert(RenderingCharacter { character: '#' });
}

#[test]
fn can_get_rendering_character_component() {
    let mut test_env = initialize();

    let entity = test_env.game.get_world_mut().spawn()
        .insert(RenderingCharacter { character: '#' }).id();

    let render_char = test_env.game.get_world_ref().entity(entity).get::<RenderingCharacter>();

    match render_char {
        None => { assert!(false) }
        Some(r_c) => { assert_eq!(r_c.character, '#') }
    }
}

#[test]
fn can_add_renderer() {
    let mut test_env = initialize();
    test_env.game.add_stage_to_schedule("Update", SystemStage::parallel()
        .with_system(character_renderer),
    )
}

#[test]
fn add_and_reset_render_target() {
    let mut test_env = initialize_with_rendered_entity_and_timing_system();

    let mut render_targets = test_env.game.get_world_mut().get_resource::<RenderTargets>();

    match render_targets {
        None => { assert!(false) }
        Some(targets) => {
            assert_eq!(0, targets.get_cloned_targets().len())
        }
    }

    test_env.game.run_schedule();
    test_env.game.run_schedule();
    test_env.game.run_schedule();

    render_targets = test_env.game.get_world_mut().get_resource::<RenderTargets>();

    match render_targets {
        None => { assert!(false) }
        Some(targets) => {
            assert_eq!(1, targets.get_cloned_targets().len())
        }
    }
}
