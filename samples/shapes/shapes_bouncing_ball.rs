// raylib-rs port by varomix

extern crate raylib;

use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib-rs [shapes] example - bouncing ball")
        .msaa_4x()
        .build();

    let mut ball_position = Vector2::new((SCREEN_WIDTH / 2) as f32, (SCREEN_HEIGHT / 2) as f32);
    let mut ball_speed = Vector2::new(5., 4.);
    let ball_radius = 20.;

    let mut pause = false;
    let mut frames_counter = 0;

    rl.set_target_fps(60);
    while !rl.window_should_close() {

        // Update //
        if rl.is_key_pressed(KEY_SPACE) {
            pause = !pause;
        }

        if !pause {
            ball_position.x += ball_speed.x;
            ball_position.y += ball_speed.y;

            // Check walls collision for bouncing
            if ball_position.x >= SCREEN_WIDTH as f32 - ball_radius
                || ball_position.x <= ball_radius {
                ball_speed.x *= -1.;
            }

            if ball_position.y >= SCREEN_HEIGHT as f32 - ball_radius
                || ball_position.y <= ball_radius {
                ball_speed.y *= -1.;
            }
        } else { frames_counter += 1; }

        // Draw //
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_circle_v(ball_position, ball_radius, Color::MAROON);
        d.draw_text("PRESS SPACE to PAUSE BALL MOVEMENT", 10, SCREEN_HEIGHT - 40, 20, Color::LIGHTGRAY);

        // On pause, we draw a blinking message
        if pause && ((frames_counter / 30) % 2) == 0 {
            d.draw_text("PAUSED", 350, 200, 30, Color::GRAY);
        }

        d.draw_fps(10, 10);

    // Debugging info to visualize ball position
        d.draw_text(&*format!("Debug info\nBall Position x: {} \nBall Position y: {}", ball_position.x, ball_position.y), 10, 30, 10, Color::BLACK);

        d.draw_text("raylib-rs port by varomix", 20, SCREEN_HEIGHT - 20, 10, Color::GRAY);
    }
}
