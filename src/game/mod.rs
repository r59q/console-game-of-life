use bevy_ecs::prelude::{Schedule, Stage, SystemStage};
use bevy_ecs::schedule::StageLabel;
use bevy_ecs::world::World;
use console_engine::{ConsoleEngine, KeyCode};
use console_engine::pixel::pxl;

use crate::inputmanager;
use crate::resources::bindings::Bindings;
use crate::resources::button_inputs::ButtonInputs;
use crate::resources::mouse_inputs::MouseInputs;
use crate::resources::render_targets::RenderTargets;

pub struct Game {
    is_started: bool,
    world: World,
    engine: ConsoleEngine,
    schedule: Schedule,
}

impl Game {
    pub fn new(min_width: u32, min_height: u32, target_fps: u32) -> Self {
        let c_engine = console_engine::ConsoleEngine::init_fill_require(min_width, min_height, target_fps);
        match c_engine {
            Ok(engine) => {
                return Self { is_started: false, world: World::new(), engine, schedule: Schedule::default() };
            }
            Err(error) => { panic!("{}", error); }
        }
    }

    pub fn start(&mut self) {
        self.init();
        self.game_loop();
        self.deinit();
    }

    pub fn get_world_mut(&mut self) -> &mut World {
        return &mut self.world;
    }

    pub fn get_world_ref(&self) -> &World {
        return &self.world;
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

    fn init(&mut self) {
        self.is_started = true;
        self.engine.check_resize(); // resize the terminal if its size has changed
    }

    fn deinit(&mut self) {
        self.is_started = false;
    }

    pub fn get_engine(&mut self) -> &ConsoleEngine {
        return &self.engine;
    }

    fn game_logic(&mut self) {
        let bindings_opt = self.get_world_ref().get_resource::<Bindings>();
        if let None = bindings_opt {
            panic!("There are no bindings")
        }
        inputmanager::capture_inputs::capture_mouse_inputs(self);
        inputmanager::capture_inputs::capture_button_inputs(self);
        inputmanager::capture_inputs::capture_axial_inputs(self);
        self.run_schedule();
    }

    fn game_render(&mut self) {
        let render_targets = self.get_world_ref().get_resource::<RenderTargets>();
        match render_targets {
            None => { panic!("NO RENDER TARGETS!!") }
            Some(targets) => {
                let cloned_targets = targets.get_cloned_targets();
                for target in cloned_targets {
                    let char = target.get_target_character();
                    let pos = target.get_target_position();
                    let int_pos = pos.to_position_int();
                    self.engine.set_pxl(int_pos.x as i32, int_pos.y as i32, pxl(char))
                }
            }
        }
        self.engine.draw();
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

            self.game_logic();
            self.game_render();
        }
    }
}