use bevy_ecs::prelude::{ResMut};
use crate::resources::timer::Timer;

pub fn timing_system(mut timer: ResMut<Timer>) {
    timer.update();
}