use super::component::Component;
use super::system::System;

pub struct Entity {
    id: String,
    components: Vec<Box<dyn Component>>,
    systems: Vec<Box<dyn System>>
}

impl Entity {
    pub fn new(id: String) -> Entity {
        return Entity { id: id, components: Vec::new(), systems: Vec::new() }
    }

    pub fn add_component(&mut self, component: impl Component) {
        self.components.push(Box::new(component))
    }

    pub fn add_system(&mut self, system: impl System) {
        self.systems.push(Box::new(system))
    }
}