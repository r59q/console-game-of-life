use crate::resources::time::Time;
use super::*;

#[test]
fn can_add_time_resource() {
    let mut test_env = initialize();

    let time_resource = Time::default();

    test_env.game.get_world().insert_resource(time_resource);

    let time = test_env.game.get_world().get_resource::<Time>();

    match time {
        None => { assert!(false) }
        Some(t) => { assert_eq!(t.seconds, 0.) }
    }

}