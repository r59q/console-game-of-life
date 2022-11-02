use bevy_ecs::change_detection::Mut;
use bevy_ecs::prelude::{Schedule, Stage, SystemStage};
use bevy_ecs::schedule::StageLabel;
use bevy_ecs::world::World;
use console_engine::{ConsoleEngine, KeyCode, MouseButton};
use console_engine::pixel::pxl;

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

    // Todo: Refactor this
    fn capture_inputs(&mut self) {
        let mouse_left_pos_down = self.engine.get_mouse_press(MouseButton::Left);
        let mouse_left_pos_up = self.engine.get_mouse_released(MouseButton::Left);
        let mouse_left_pos_held = self.engine.get_mouse_held(MouseButton::Left);

        let mouse_right_pos_down = self.engine.get_mouse_press(MouseButton::Right);
        let mouse_right_pos_up = self.engine.get_mouse_released(MouseButton::Right);
        let mouse_right_pos_held = self.engine.get_mouse_held(MouseButton::Right);

        let a_key_down = self.engine.is_key_pressed(KeyCode::Char('a'));
        let a_key_held = self.engine.is_key_held(KeyCode::Char('a'));
        let a_key_up = self.engine.is_key_released(KeyCode::Char('a'));

        let w_key_down = self.engine.is_key_pressed(KeyCode::Char('w'));
        let w_key_held = self.engine.is_key_held(KeyCode::Char('w'));
        let w_key_up = self.engine.is_key_released(KeyCode::Char('w'));

        let s_key_down = self.engine.is_key_pressed(KeyCode::Char('s'));
        let s_key_held = self.engine.is_key_held(KeyCode::Char('s'));
        let s_key_up = self.engine.is_key_released(KeyCode::Char('s'));

        let d_key_down = self.engine.is_key_pressed(KeyCode::Char('d'));
        let d_key_held = self.engine.is_key_held(KeyCode::Char('d'));
        let d_key_up = self.engine.is_key_released(KeyCode::Char('d'));

        // let inputs = self.get_world_mut().get_resource_mut::<Inputs>();
        // if let Some(mut inputs) = inputs {
            // if let Some(mouse_left_pos_down) = mouse_left_pos_down {
            //     inputs.register_mouse_press(crate::resources::inputs::MouseButton::LEFT, DOWN, mouse_left_pos_down);
            // }
            // if let Some(mouse_left_pos_held) = mouse_left_pos_held {
            //     inputs.register_mouse_press(crate::resources::inputs::MouseButton::LEFT, HELD, mouse_left_pos_held);
            // }
            // if let Some(mouse_left_pos_up) = mouse_left_pos_up {
            //     inputs.register_mouse_press(crate::resources::inputs::MouseButton::LEFT, UP, mouse_left_pos_up);
            // }

            // if let Some(mouse_right_pos_down) = mouse_right_pos_down {
            //     inputs.register_mouse_press(crate::resources::inputs::MouseButton::RIGHT, DOWN, mouse_right_pos_down);
            // }
            // if let Some(mouse_right_pos_held) = mouse_right_pos_held {
            //     inputs.register_mouse_press(crate::resources::inputs::MouseButton::RIGHT, HELD, mouse_right_pos_held);
            // }
            // if let Some(mouse_right_pos_up) = mouse_right_pos_up {
            //     inputs.register_mouse_press(crate::resources::inputs::MouseButton::RIGHT, UP, mouse_right_pos_up);
            // }

            // if a_key_down {
            //     inputs.register_key_press(KeyCode::Char('a'), DOWN)
            // }
            // if a_key_held {
            //     inputs.register_key_press(KeyCode::Char('a'), HELD)
            // }
            // if a_key_up {
            //     inputs.register_key_press(KeyCode::Char('a'), UP)
            // }

            // if w_key_down {
            //     inputs.register_key_press(KeyCode::Char('w'), DOWN)
            // }
            // if w_key_held {
            //     inputs.register_key_press(KeyCode::Char('w'), HELD)
            // }
            // if w_key_up {
            //     inputs.register_key_press(KeyCode::Char('w'), UP)
            // }

            // if d_key_down {
            //     inputs.register_key_press(KeyCode::Char('d'), DOWN)
            // }
            // if d_key_held {
            //     inputs.register_key_press(KeyCode::Char('d'), HELD)
            // }
            // if d_key_up {
            //     inputs.register_key_press(KeyCode::Char('d'), UP)
            // }

            // if s_key_down {
            //     inputs.register_key_press(KeyCode::Char('s'), DOWN)
            // }
            // if s_key_held {
            //     inputs.register_key_press(KeyCode::Char('s'), HELD)
            // }
            // if s_key_up {
            //     inputs.register_key_press(KeyCode::Char('s'), UP)
            // }
        // }
    }

    fn game_logic(&mut self) {
        self.capture_inputs();

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