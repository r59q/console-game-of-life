use components::{position::Position, velocity::Velocity};
use game::Game;
use crate::resources::time::Time;

mod game;
mod components;
mod systems;
mod resources;

fn main() {
    let time = Time::default();

    println!("{}", time.seconds);

    let mut game: Game = Game::new();
    let mut player_entity = 
        game.get_world().spawn();

    player_entity
        .insert(Position {x: 0., y: 0.})
        .insert(Velocity {x: 0., y:0.});

    game.start();
}

#[cfg(test)]
mod tests;