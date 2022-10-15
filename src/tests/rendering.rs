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
