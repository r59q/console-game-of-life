use self::entity::Entity;

pub mod entity;
pub mod component;

pub struct ECS {
    entities: Vec<Entity>
}

impl ECS {
    pub fn new() -> ECS {
        return ECS { entities: Vec::new() }
    }
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity)
    }
    pub fn get_entity(&self, id: String) -> Option<&Entity> {
        for entity in self.entities.iter() {
            if entity.get_id() == id {
                return Some(entity);
            }
        }
        return None;
    }
}