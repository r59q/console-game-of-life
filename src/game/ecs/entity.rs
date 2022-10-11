use super::component::Component;
use super::system::System;

pub struct Entity {
    id: String,
    components: Vec<Box<dyn Component>>,
    systems: Vec<Box<dyn System>>
}

impl Entity {
    pub fn new(id: String) -> Entity {
        return Entity { id, components: Vec::new(), systems: Vec::new() }
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.components.push(component)
    }

    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.push(system)
    }
}