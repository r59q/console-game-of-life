use components::{position::Position, velocity::Velocity};
use game::Game;

mod game;
mod components;

fn main() {
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