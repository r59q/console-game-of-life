use crate::game::ecs::component::Component;

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
}