use crate::components::position::{Position, PositionInt};
use crate::components::rendering_character::RenderingCharacter;

pub struct RenderTarget {
    position: Position,
    character: char,
}

impl RenderTarget {
    pub fn new(position: Position, character: char) -> Self {
        return Self {position, character}
    }
}

pub struct RenderTargets {
    targets: Vec<RenderTarget>,
}

impl RenderTargets {
    pub fn new() -> Self {
        return RenderTargets {
            targets: Vec::new()
        };
    }

    pub fn add(&mut self, target: RenderTarget) {
        self.targets.push(target);
    }
}

#[cfg(test)]
mod test {
    use crate::components::position::Position;
    use crate::components::rendering_character::RenderingCharacter;
    use crate::resources::render_targets::{RenderTarget, RenderTargets};

    fn can_add_to_targets() {
        let mut targets = RenderTargets::new();
        let position = Position { x: 0., y: 0. };

        let new_target = RenderTarget {position, character: 't' };

        assert_eq!(targets.targets.len(), 0);
        targets.add(new_target);
        assert_eq!(targets.targets.len(), 1);
    }
}