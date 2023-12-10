use console_engine_TC_FIX::Color;

use super::ui_layer::UILayer;

pub fn insert_tutorial_text(ui_layer: &mut UILayer) -> () {
    ui_layer.insert_text(
        "*************************************************",
        (1, 1),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "*             Conway's Game of Life             *",
        (1, 2),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "*                    by r59q                    *",
        (1, 3),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "*              --   Help menu   --              *",
        (1, 4),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "*                                               *",
        (1, 5),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "* ESC              - Closes program             *",
        (1, 6),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "* P                - Pause / Unpause            *",
        (1, 7),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "* Left click       - Place / remove cell        *",
        (1, 8),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "* Right click+drag - Drags simulation space     *",
        (1, 9),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "* H                - Toggle help menu           *",
        (1, 10),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "* Z                - Decrease simulation speed  *",
        (1, 11),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "* X                - Increase simulation speed  *",
        (1, 12),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "* R                - Randomize simulation       *",
        (1, 13),
        Color::Yellow,
    );
    ui_layer.insert_text(
        "*************************************************",
        (1, 14),
        Color::Yellow,
    );
    ui_layer.set_pixel((3, 6), 'E', Color::Red);
    ui_layer.set_pixel((4, 6), 'S', Color::Red);
    ui_layer.set_pixel((5, 6), 'C', Color::Red);
    ui_layer.set_pixel((3, 7), 'P', Color::Red);
    ui_layer.insert_text("Left click", (3, 8), Color::Red);
    ui_layer.insert_text("Right click+drag", (3, 9), Color::Red);
    ui_layer.set_pixel((3, 10), 'H', Color::Red);
    ui_layer.set_pixel((3, 11), 'Z', Color::Red);
    ui_layer.set_pixel((3, 12), 'X', Color::Red);
    ui_layer.set_pixel((3, 13), 'R', Color::Red);
}

pub fn clear_tutorial_text(ui_layer: &mut UILayer) {
    for x in 1..50 {
        for y in 1..15 {
            ui_layer.set_pixel((x, y), ' ', Color::Blue)
        }
    }
}

pub fn insert_help_toggle_hint(ui_layer: &mut UILayer) -> () {
    ui_layer.insert_text("H - Help menu", (1, 1), Color::Yellow);
    ui_layer.set_pixel((1, 1), 'H', Color::Red)
}
