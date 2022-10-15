use bevy_ecs::prelude::{Query, Res};
use crate::components::position::Position;
use crate::components::velocity::Velocity;
use crate::resources::timer::Timer;

pub fn movement_system(time: Res<Timer>, mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in &mut query {
        position.x += velocity.x * time.delta_time.as_secs_f64();
        position.y += velocity.y * time.delta_time.as_secs_f64();
    }
}