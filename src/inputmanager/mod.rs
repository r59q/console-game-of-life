use console_engine::{ConsoleEngine, KeyCode};

pub enum Axis {
    Horizontal,
    Vertical
}

pub struct MovementDelta {
    dx: i32,
    dy: i32
}

pub fn get_axis(axis: Axis, engine: &ConsoleEngine) -> i32 {
    match axis {
        Axis::Horizontal => 
        if engine.is_key_pressed(KeyCode::Char('a')) {
            return -1;
        } else if engine.is_key_pressed(KeyCode::Char('d')) {
            return 1;
        } else { return 0; }
        Axis::Vertical => if engine.is_key_pressed(KeyCode::Char('w')) {
            return -1;
        } else if engine.is_key_pressed(KeyCode::Char('s')) {
            return 1;
        } else { return 0; },
    }
}