use game::{ecs::entity::Entity, Game, init_game};
use crate::components::transform::{Position};

mod game;
mod components;
mod tests;

fn main() {
    let mut game: Game = init_game();

    let mut entity: Entity = Entity::new("Player".to_string());

    let pos : Position = Position::zero();

    entity.add_component(Box::new(pos));

    game.add_entity(entity);
    
    game.start();
}
