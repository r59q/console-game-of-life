use game::Game;

mod game;
mod components;

fn main() {
    let mut game: Game = Game::new();
    game.start();
}

#[cfg(test)]
mod tests;