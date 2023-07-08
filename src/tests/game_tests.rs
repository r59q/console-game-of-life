use crate::prefabs::Prefabs;

use super::initialize;

#[test]
fn can_place_prefab() {
    let mut test_env = initialize();
    test_env.game.spawn_prefab(Prefabs::PLACEABLE_CELL);
    let entity_count = test_env.game.get_world_ref().entities().len();
    assert_eq!(entity_count, 1)
}

#[test]
fn can_place_prefab_at_position() {
    let mut test_env = initialize();
    let entity = test_env.game.spawn_prefab(Prefabs::PLACEABLE_CELL);
    let entity_count = test_env.game.get_world_ref().entities().len();
    assert_eq!(entity_count, 1);
}