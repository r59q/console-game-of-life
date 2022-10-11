mod inputmanager;

use console_engine::{pixel, KeyModifiers};
use console_engine::{KeyCode};
use inputmanager::Axis;

fn main() {
    // initializes a screen filling the terminal with a target of 30 frames per second
    let mut engine = console_engine::ConsoleEngine::init_fill_require(30, 20, 30).unwrap();

    let mut pos_x = 3;
    let mut pos_y = 3;

    let run_speed_multiplier = 2;

    let mut is_running = false;
    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.check_resize(); // resize the terminal if its size has changed
        if engine.is_key_pressed(KeyCode::Char('q')) {
            // if the user presses 'q' :
            break; // exits app
        }
        
        if engine.is_key_pressed(KeyCode::Char('r'))  {
            is_running = !is_running;
        }

        let mut speed_multiplier = 1; 
        if is_running {
            speed_multiplier = run_speed_multiplier;
        }

        pos_x += speed_multiplier * inputmanager::get_axis(Axis::Horizontal, &engine);
        pos_y += speed_multiplier * inputmanager::get_axis(Axis::Vertical, &engine);

        engine.clear_screen();

        engine.set_pxl(pos_x, pos_y, pixel::pxl('@'));

        let run_walk_indicator = if is_running { "Running" } else {"Walking"};
        engine.print(0, 0, run_walk_indicator);

        engine.draw(); // draw the screen
    }
}
