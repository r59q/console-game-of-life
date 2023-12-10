use std::collections::HashMap;

use console_engine_TC_FIX::Color;

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

    pub fn get_pixels(&self) -> &HashMap<(u32, u32), UILayerPixel> {
        &self.pixels
    }

    pub fn set_pixel(&mut self, position: (u32, u32), value: char, color: Color) -> () {
        self.pixels
            .insert(position, UILayerPixel::new(value, color));
    }

    pub fn get_pixel(&self, pos: (u32, u32)) -> Option<&UILayerPixel> {
        self.pixels.get(&pos)
    }

    pub fn insert_text(&mut self, text: &str, offset: (u32, u32), color: Color) -> () {
        for (i, c) in text.chars().enumerate() {
            let offset_x: u32 = offset.0 + i as u32; // This is fine, just UI elements.
            self.set_pixel((offset_x, offset.1), c, color)
        }
    }
}

#[cfg(test)]
mod test {
    use console_engine_TC_FIX::Color;

    use super::UILayer;

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

    #[test]
    fn can_insert_text() {
        let mut ui_layer = UILayer::new();
        let text = "1 2 3 4 5";
        let offset = (2, 2);
        ui_layer.insert_text(text, offset, Color::Green);
        assert_eq!(9, ui_layer.get_pixels().len());
        assert!(ui_layer.get_pixel((5, 2)).is_some());
        assert_eq!(' ', *ui_layer.get_pixel((5, 2)).unwrap().get_character());
    }
}
