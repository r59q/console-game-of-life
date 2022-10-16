use crate::components::position::{Position};

#[derive(Clone)]
pub struct RenderTarget {
    position: Position,
    character: char,
}

impl RenderTarget {
    pub fn new(position: Position, character: char) -> Self {
        return Self {position, character}
    }
    pub fn get_target_character(&self) -> char {
        return self.character;
    }

    pub fn get_target_position(&self) -> Position {
        return self.position;
    }
}

#[derive(Clone)]
pub struct RenderTargets {
    targets: Vec<RenderTarget>,
}

impl RenderTargets {
    pub fn new() -> Self {
        return RenderTargets {
            targets: Vec::new()
        };
    }

    pub fn reset(&mut self) {
        self.targets.clear();
    }

    pub fn get_cloned_targets(&self) -> Vec<RenderTarget> {
        return self.targets.to_owned();
    }

    pub fn add(&mut self, target: RenderTarget) {
        self.targets.push(target);
    }
}

#[cfg(test)]
mod test {
    use crate::components::position::Position;
    use crate::resources::render_targets::{RenderTarget, RenderTargets};

    #[test]
    fn can_add_to_targets() {
        let mut targets = RenderTargets::new();
        let position = Position { x: 0., y: 0. };

        let new_target = RenderTarget {position, character: 't' };

        assert_eq!(targets.targets.len(), 0);
        targets.add(new_target);
        assert_eq!(targets.targets.len(), 1);
    }

    #[test]
    fn can_get_targets() {
        let mut targets = RenderTargets::new();
        let position = Position { x: 0., y: 0. };

        let new_target = RenderTarget {position, character: 't' };

        assert_eq!(targets.targets.len(), 0);
        targets.add(new_target);
        assert_eq!(targets.get_cloned_targets().len(), 1);
    }

    #[test]
    fn can_get_target() {
        let mut targets = RenderTargets::new();
        let position = Position { x: 1., y: 2. };

        let new_target = RenderTarget {position, character: 't' };

        assert_eq!(targets.targets.len(), 0);
        targets.add(new_target);
        assert_eq!(targets.get_cloned_targets().get(0).unwrap().get_target_character(), 't');
        assert_eq!(targets.get_cloned_targets().get(0).unwrap().get_target_position().x, 1.);
        assert_eq!(targets.get_cloned_targets().get(0).unwrap().get_target_position().y, 2.);
    }

    #[test]
    fn can_reset_targets() {
        let mut targets = RenderTargets::new();
        let position = Position { x: 0., y: 0. };
        let new_target = RenderTarget {position, character: 't' };

        targets.add(new_target);
        assert_eq!(targets.targets.len(), 1);
        targets.reset();
        assert_eq!(targets.targets.len(), 0);
    }
}