// raylib-rs port by varomix

extern crate raylib;

use raylib::prelude::*;
use raylib::consts::MouseButton::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib-rs [shapes] example - cubic-bezier lines")
        .msaa_4x()
        .build();

    let mut start = Vector2::new(0., 0.);
    let mut end = Vector2::new(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    rl.set_target_fps(60);
    while !rl.window_should_close() {

        // Update
        if rl.is_mouse_button_down(MOUSE_LEFT_BUTTON) {
            start = rl.get_mouse_position();
        } else if rl.is_mouse_button_down(MOUSE_RIGHT_BUTTON) {
            end = rl.get_mouse_position();
        }

        // Draw //
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_text("USE MOUSE LEFT-RIGHT CLICK to DEFINE LINE START and END POINTS", 15, 20, 20, Color::GRAY);

        d.draw_line_bezier(start, end, 2., Color::RED);

        d.draw_text("raylib-rs port by varomix", 20, SCREEN_HEIGHT - 20, 10, Color::GRAY);
    }
}
