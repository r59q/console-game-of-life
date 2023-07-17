use std::collections::HashMap;

use console_engine::Color;

use super::ui_layer_pixel::UILayerPixel;

#[derive(Debug)]
pub struct UILayer {
    pixels: HashMap<(u32, u32), UILayerPixel>,
}

impl UILayer {
    pub fn new() -> UILayer {
        UILayer {
            pixels: HashMap::new(),
        }
    }

    pub fn get_pixels(self) -> HashMap<(u32, u32), UILayerPixel> {
        self.pixels
    }

    pub fn set_pixel(&mut self, position: (u32, u32), value: char, color: Color) -> () {
        self.pixels.insert(position, UILayerPixel {});
    }

    pub fn get_pixel(&self, pos: (u32, u32)) -> Option<&UILayerPixel> {
        self.pixels.get(&pos)
    }
}

#[cfg(test)]
mod test {
    use bevy_ecs::world::World;
    use console_engine::Color;

    use crate::{game::Game, tests::TestEnv};

    use super::UILayer;

    fn initialize() -> TestEnv {
        let entity = World::new().spawn().id();
        TestEnv {
            game: Game::new(1, 1, 1),
            entity: entity,
        }
    }

    #[test]
    fn can_create_ui_layer() {
        UILayer::new();
    }

    #[test]
    fn can_get_ui_layer_pixels() {
        let ui_layer = UILayer::new();

        ui_layer.get_pixels();
    }

    #[test]
    fn can_set_ui_layer_pixel() {
        let mut ui_layer = UILayer::new();
        ui_layer.set_pixel((1, 1), 't', Color::Green);
        ui_layer.set_pixel((1, 1), 't', Color::Green);
        assert_eq!(1, ui_layer.get_pixels().len());
    }

    #[test]
    fn can_get_pixel() {
        let mut ui_layer = UILayer::new();
        ui_layer.set_pixel((1, 1), 't', Color::Green);
        assert!(ui_layer.get_pixel((1, 1)).is_some());
        assert!(ui_layer.get_pixel((0, 0)).is_none());
    }
}
