use console_engine::Color;

#[derive(Debug, Clone, Copy)]
pub struct UILayerPixel {
    character: char,
    color: Color,
}

impl UILayerPixel {
    pub fn new(character: char, color: Color) -> UILayerPixel {
        UILayerPixel { character, color }
    }

    pub fn get_character(&self) -> &char {
        &self.character
    }

    pub fn get_color(&self) -> Color {
        self.color
    }
}

#[cfg(test)]
mod test {
    use console_engine::Color;

    use super::UILayerPixel;

    #[test]
    fn can_get_character() {
        let pixel = UILayerPixel::new('x', Color::Green);
        assert_eq!(&'x', pixel.get_character())
    }

    fn can_get_color() {
        let pixel = UILayerPixel::new('x', Color::Yellow);
        assert_eq!(Color::Yellow, pixel.get_color())
    }
}
