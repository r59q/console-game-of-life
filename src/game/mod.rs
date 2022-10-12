use console_engine::{ConsoleEngine, KeyCode};

use self::ecs::{entity::Entity, ECS};

pub mod ecs;

pub struct Game {
    is_started: bool,
    ecs: ECS,
    engine: ConsoleEngine,
}

pub fn init_game() -> Game {
    let c_engine = console_engine::ConsoleEngine::init_fill_require(30, 20, 60).unwrap();
    return Game { is_started:false, ecs:ECS::new(), engine: c_engine }
}

impl Game {
    pub fn start(&mut self) {
        self.init();
        self.game_loop();
        self.denit();
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.ecs.add_entity(entity)
    }
    pub fn get_entity_by_id(&self, id: String) -> Option<&Entity> {
        return self.ecs.get_entity(id);
    }

    fn init(&mut self) {
        self.is_started = true;
        self.engine.check_resize(); // resize the terminal if its size has changed
    }
    fn denit(&mut self) {
        self.is_started = false;
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