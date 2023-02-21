use crate::Game;

pub struct GameRefs {
    game: Game
}
impl GameRefs {
    pub(crate) fn new(game: Game) -> GameRefs {
        return GameRefs { game  }
    }

    pub(crate) fn get_game(&self) -> &Game {
        return &self.game;
    }

}

#[cfg(test)]
mod test {
    use crate::game::Game;

    use super::GameRefs;


    #[test]
    fn can_create_refs() {
        let game: Game = Game::new(10, 10, 10); 
        GameRefs::new(game);
    }

    #[test]
    fn can_get_game_ref() {
        let game: Game = Game::new(10, 10, 10); 
        let refs = GameRefs::new(game);
        refs.get_game();
    }
 }