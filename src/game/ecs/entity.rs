use crate::game::ecs::component::Component;
use std::any::type_name;

pub struct Entity {
    id: String,
    components: Vec<Box<dyn Component>>,
}

impl Entity {
    pub fn new(id: String) -> Entity {
        return Entity { id, components: Vec::new() };
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component)
    }

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    pub fn get_component<T: Component>(&self, _: T) -> Option<T> {
        for component in self.components.iter() {
            if Self::type_of(component) == type_name::<T>() {
                return Some(component)
            }
        }
        return None;
    }

    pub fn get_id(&self) -> &str {
        return &self.id
    }
}