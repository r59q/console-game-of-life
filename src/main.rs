use game::Game;

mod game;
mod components;
mod tests;

fn main() {
    let mut game: Game = Game::new();
    game.start();
}
