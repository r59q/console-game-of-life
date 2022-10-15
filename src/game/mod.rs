use bevy_ecs::prelude::{Schedule, Stage, SystemStage};
use bevy_ecs::schedule::StageLabel;
use bevy_ecs::world::World;
use console_engine::{ConsoleEngine, KeyCode};


pub struct Game {
    is_started: bool,
    world: World,
    engine: ConsoleEngine,
    schedule: Schedule
}

impl Game {
    pub fn new() -> Self {
        let c_engine = console_engine::ConsoleEngine::init_fill_require(30, 20, 60).unwrap();
        return Self { is_started: false, world: World::new(), engine: c_engine, schedule: Schedule::default() };
    }
    pub fn start(&mut self) {
        self.init();
        self.game_loop();
        self.deinit();
    }
    fn init(&mut self) {
        self.is_started = true;
        self.engine.check_resize(); // resize the terminal if its size has changed
    }
    fn deinit(&mut self) {
        self.is_started = false;
    }

    pub fn get_world_mut(&mut self) -> &mut World {
        return &mut self.world
    }

    pub fn get_world_ref(&self) -> &World {
        return &self.world
    }

    pub fn set_schedule(&mut self, schedule: Schedule) {
        self.schedule = schedule;
    }

    pub fn get_schedule_mut(&mut self) -> &mut Schedule {
        &mut self.schedule
    }

    pub fn run_schedule(&mut self) {
        self.schedule.run(&mut self.world)
    }

    pub fn add_stage_to_schedule(&mut self, stage_label: impl StageLabel, stage: SystemStage) -> () {
        self.schedule.add_stage(stage_label, stage);
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