use crate::Game;

pub struct GameRefs<'w> {
    game: &'w Game
}
impl GameRefs<'_> {
    pub(crate) fn new(game: &Game) -> GameRefs {
        return GameRefs { game  }
    }

}

#[cfg(test)]
mod test { }