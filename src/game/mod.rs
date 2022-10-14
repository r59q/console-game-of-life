use bevy_ecs::world::World;
use console_engine::{ConsoleEngine, KeyCode};


pub struct Game {
    is_started: bool,
    world: World,
    engine: ConsoleEngine,
}

impl Game {
    pub fn new() -> Self {
        let c_engine = console_engine::ConsoleEngine::init_fill_require(30, 20, 60).unwrap();
        return Game { is_started:false, world:World::new(), engine: c_engine }
    }
    pub fn start(&mut self) {
        self.init();
        self.game_loop();
        self.denit();
    }
    fn init(&mut self) {
        self.is_started = true;
        self.engine.check_resize(); // resize the terminal if its size has changed
    }
    fn denit(&mut self) {
        self.is_started = false;
    }
    pub fn get_world(&mut self) -> &mut World {
        return &mut self.world
    }
    fn game_loop(&mut self) {
        loop {
            self.engine.wait_frame(); // wait for next frame + capture inputs

            if self.engine.is_key_pressed(KeyCode::Esc) {
                break;
            }
            if self.engine.is_key_pressed(KeyCode::Char('æ')) {
                break;
            }
            self.engine.clear_screen();
        }
    }
}