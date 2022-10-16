use bevy_ecs::prelude::Component;

#[derive(Component, Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn to_position_int(&self) -> PositionInt {
        return PositionInt {
            x: self.x.round() as i64,
            y: self.y.round() as i64
        };
    }
}

#[derive(Component, Clone, Copy)]
pub struct PositionInt {
    pub x: i64,
    pub y: i64,
}

#[cfg(test)]
mod test {
    use crate::components::position::Position;

    #[test]
    fn can_convert_to_int() {
        let test_pos = Position { x: 10.8, y: 1.1 };
        let test_int_pos = test_pos.to_position_int();

        assert_eq!(test_int_pos.x, 11);
        assert_eq!(test_int_pos.y, 1);
    }
}