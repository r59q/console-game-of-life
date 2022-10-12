#[cfg(test)]
// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

struct TestEnv {
    pub game: Game,
    pub entity: Entity
}

fn initialize() -> TestEnv {
    return TestEnv { game: init_game(), entity: Entity::new("test_id".to_string()) }
}

#[test]
fn can_add_entity() {
    let mut test_env = initialize();
    let added_id: String = test_env.entity.get_id().to_string();
    test_env.game.add_entity(test_env.entity);

    let added_entity = test_env.game.get_entity_by_id(added_id);
    match added_entity {
        None    => { assert!(false) }
        Some(_) => { assert!(true) }
    }
}

#[test]
fn can_add_component() {
    let test_env = initialize();
    let mut entity = test_env.entity;
    entity.add_component(Box::new(Position::zero()));
    assert_ne!(0, entity.get_components().iter().len());
}
