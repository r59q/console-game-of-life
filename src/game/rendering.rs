use std::collections::HashMap;

use console_engine::pixel;

use crate::resources::{
    render_targets::RenderTargets,
    ui::{ui_layer::UILayer, ui_layer_pixel::UILayerPixel},
    view_offset::ViewOffset,
};

use super::Game;

impl Game {
    pub(super) fn game_render(&mut self) {
        self.panic_if_resources_not_present();
        self.entity_render_pass();
        self.ui_layer_render_pass();
        self.engine.draw();
    }

    fn panic_if_resources_not_present(&self) {
        let view_offset = self.get_world_ref().get_resource::<ViewOffset>();
        let render_targets = self.get_world_ref().get_resource::<RenderTargets>();
        let ui_layer = self.get_world_ref().get_resource::<UILayer>();
        if render_targets.is_none() {
            panic!("NO RENDER TARGETS!!");
        }
        if view_offset.is_none() {
            panic!("NO VIEW OFFSET!");
        }
        if ui_layer.is_none() {
            panic!("NO UI LAYER RESOURCE");
        }
    }

    fn ui_layer_render_pass(&mut self) -> () {
        let ui_layer = self
            .get_world_ref()
            .get_resource::<UILayer>()
            .expect("No UI layer could be found");
        let pixels: HashMap<(u32, u32), UILayerPixel> = ui_layer.get_pixels().clone();
        for (pos, pixel) in pixels {
            // makes spaces transparent
            if pixel.get_character() != &' ' {
                self.engine.set_pxl(
                    pos.0 as i32,
                    pos.1 as i32,
                    pixel::pxl_fg(*pixel.get_character(), pixel.get_color()),
                )
            }
        }
    }

    fn entity_render_pass(&mut self) -> () {
        let view_offset = self.get_world_ref().get_resource::<ViewOffset>();
        let render_targets = self.get_world_ref().get_resource::<RenderTargets>();

        let targets = render_targets.unwrap();
        let (x_offset, y_offset) = view_offset.unwrap().get_offset();

        let mut cloned_targets = targets.get_cloned_targets();
        cloned_targets.reverse(); // Reverse it such that entities added later will have priority
        for target in cloned_targets {
            let char = target.get_target_character();
            let color = target.get_target_color();
            let pos = target.get_target_position();
            let mut int_pos = pos.to_position_int();
            int_pos.x = int_pos.x - x_offset;
            int_pos.y = int_pos.y - y_offset;
            self.engine.set_pxl(
                int_pos.x as i32,
                int_pos.y as i32,
                pixel::pxl_fg(char, color),
            )
        }
    }
}
