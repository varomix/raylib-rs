// raylib-rs port by varomix

extern crate raylib;

use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib-rs [shapes] example - raylib logo using shapes")
        .build();

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        // Draw //
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_rectangle(SCREEN_WIDTH / 2 - 128, SCREEN_HEIGHT / 2 - 128, 256, 256, Color::BLACK);
        d.draw_rectangle(SCREEN_WIDTH / 2 - 112, SCREEN_HEIGHT / 2 - 112, 224, 224, Color::WHITE);
        d.draw_text("raylib", SCREEN_WIDTH / 2 - 44, SCREEN_HEIGHT / 2 + 10, 50, Color::BLACK);
        d.draw_text("rust", SCREEN_WIDTH / 2 - 14, SCREEN_HEIGHT / 2 + 48, 50, Color::RED);

        d.draw_text("This is NOT a texture!", 350, 370, 10, Color::GRAY);

        d.draw_text("raylib-rs port by varomix", 20, SCREEN_HEIGHT - 20, 10, Color::GRAY);
    }
}
