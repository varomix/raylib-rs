// raylib-rs port by varomix

extern crate raylib;

use raylib::prelude::*;

const MAX_FRAME_SPEED: i32 = 15;
const MIN_FRAME_SPEED: i32 = 1;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

fn main() {
    use raylib::consts::KeyboardKey::*;

    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib-rs [texture] example - texture rectangle")
        .build();

    // Loading sprite sheet
    let scarfy = rl.load_texture(&thread, "static/scarfy.png").expect("Could not load Textture from Image");

    let position = Vector2::new(350., 280.);
    let mut frame_rec = Rectangle::new(0., 0., (scarfy.width() / 6) as f32, scarfy.height() as f32);
    let mut current_frame = 0;

    let mut frames_counter = 0;
    let mut frames_speed = 8;       // Spritesheet Frames per second (FPS)

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        frames_counter += 1;

        if frames_counter >= (60 / frames_speed) {
            frames_counter = 0;
            current_frame += 1;

            // go back to first frame
            if current_frame > 5 {
                current_frame = 0;
            }

            frame_rec.x = (current_frame * scarfy.width() / 6) as f32;
        }

        if rl.is_key_pressed(KEY_RIGHT) {
            frames_speed += 1;
        } else if rl.is_key_pressed(KEY_LEFT) {
            frames_speed -= 1;
        }

        if frames_speed > MAX_FRAME_SPEED {
            frames_speed = MAX_FRAME_SPEED;
        } else if frames_speed < MIN_FRAME_SPEED {
            frames_speed = MIN_FRAME_SPEED;
        }

        // Draw //
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        // Spritesheet highlight selected frame
        d.draw_texture(&scarfy, 15, 40, Color::WHITE);
        d.draw_rectangle_lines(15, 40, scarfy.width(), scarfy.height(), Color::LIME);
        d.draw_rectangle_lines(15 + frame_rec.x as i32, 40 + frame_rec.y as i32, frame_rec.width as i32, frame_rec.height as i32, Color::RED);

        // texts
        d.draw_text("FRAME SPEED: ", 165, 210, 10, Color::DARKGRAY);
        d.draw_text(&*format!("{} FPS", frames_speed), 575, 210, 10, Color::DARKGRAY);
        d.draw_text("PRESS RIGHT/LEFT KEYS to CHANGE SPEED!", 290, 240, 10, Color::DARKGRAY);

        // draw speed squares
        for i in 0..MAX_FRAME_SPEED {
            if i < frames_speed {
                d.draw_rectangle(250 + 21 * i, 205, 20, 20, Color::RED);
            }
            d.draw_rectangle_lines(250 + 21 * i, 205, 20, 20, Color::MAROON);
        }

        // Scarfy
        d.draw_texture_rec(&scarfy, frame_rec, &position, Color::WHITE);

        d.draw_text("(c) Scarfy sprite by Eiden Marsal", SCREEN_WIDTH - 200, SCREEN_HEIGHT - 20, 10, Color::GRAY);
        d.draw_text("raylib-rs port by varomix", 20, SCREEN_HEIGHT - 20, 10, Color::GRAY);
    }
}
