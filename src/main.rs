use game::{ecs::entity::Entity, Game, init_game};
mod game;
mod components;
fn main() {
    let mut game: Game = init_game();

    let entity: Entity = Entity::new("Player".to_string());

    game.add_entity(entity);
    
    game.start();
}
