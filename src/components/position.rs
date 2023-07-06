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
            y: self.y.round() as i64,
        };
    }

    pub fn set_position(&mut self, new_x: f64, new_y: f64) -> () {
        self.x = new_x;
        self.y = new_y;
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

    #[test]
    fn can_set_position() {
        let mut test_pos = Position { x: 0., y: 0. };

        test_pos.set_position(3., 5.);
        assert_eq!(test_pos.x, 3.);
        assert_eq!(test_pos.y, 5.);
    }
}
