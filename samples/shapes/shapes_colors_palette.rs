// raylib-rs port by varomix

extern crate raylib;

use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

const MAX_COLORS_COUNT: usize = 21;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib-rs [shapes] example - colors palette")
        .build();

    let colors: [Color; MAX_COLORS_COUNT] = [
        Color::DARKGRAY, Color::MAROON, Color::ORANGE, Color::DARKGREEN, Color::DARKBLUE, Color::DARKPURPLE, Color::DARKBROWN,
        Color::GRAY, Color::RED, Color::GOLD, Color::LIME, Color::BLUE, Color::VIOLET, Color::BROWN, Color::LIGHTGRAY, Color::PINK, Color::YELLOW,
        Color::GREEN, Color::SKYBLUE, Color::PURPLE, Color::BEIGE
    ];

    let color_names: [&str; 21] = [
        "DARKGRAY", "MAROON", "ORANGE", "DARKGREEN", "DARKBLUE", "DARKPURPLE",
        "DARKBROWN", "GRAY", "RED", "GOLD", "LIME", "BLUE", "VIOLET", "BROWN",
        "LIGHTGRAY", "PINK", "YELLOW", "GREEN", "SKYBLUE", "PURPLE", "BEIGE"
    ];

    let mut colors_recs: [Rectangle; MAX_COLORS_COUNT] = Default::default();

    // Fills colors_recs data (for every rectangle)
    for i in 0..MAX_COLORS_COUNT {
        colors_recs[i].x = (20 + 100 * (i % 7) + 10 * (i % 7)) as f32;
        colors_recs[i].y = (80 + 100 * (i / 7) + 10 * (i / 7)) as f32;
        colors_recs[i].width = 100.;
        colors_recs[i].height = 100.;
    }

    // Color state: 0-DEFAULT, 1-MOUSE_HOVER
    let mut color_state: [i32; MAX_COLORS_COUNT] = Default::default();

    let mut mouse_point = Vector2::new(0., 0.);

    let mut press_space = false;

    rl.set_target_fps(60);
    while !rl.window_should_close() {

        // Update
        mouse_point = rl.get_mouse_position();

        if rl.is_key_down(KEY_SPACE) {
            press_space = true;
        } else { press_space = false }

        for i in 0..MAX_COLORS_COUNT {
            if colors_recs[i].check_collision_point_rec(&mouse_point) {
                color_state[i] = 1;
            } else {
                color_state[i] = 0;
            }
        }

        // Draw //
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_text("Raylib colors palette", 28, 42, 20, Color::BLACK);
        d.draw_text("Press SPACE to see all colors", SCREEN_WIDTH - 180, SCREEN_HEIGHT - 40, 10, Color::GRAY);

        for i in 0..MAX_COLORS_COUNT {     // Draw all rectangles
            d.draw_rectangle_rec(colors_recs[i], Color::fade(
                &colors[i],
                if color_state[i] == 1 { 0.6 } else { 1.0 },
            ));

            if press_space || color_state[i] == 1 {
                d.draw_rectangle(colors_recs[i].x as i32,
                                 (colors_recs[i].y + colors_recs[i].height - 26.) as i32,
                                 colors_recs[i].width as i32, 20, Color::BLACK,
                );

                d.draw_rectangle_lines_ex(
                    colors_recs[i],
                    6,
                    Color::fade(&Color::BLACK, 0.3)
                );

                d.draw_text(
                    color_names[i],
                    colors_recs[i].x as i32 + colors_recs[i].width as i32 - measure_text(color_names[i], 10) - 12,
                    (colors_recs[i].y + colors_recs[i].height - 20.) as i32,
                    10,
                    colors[i]
                );

            }
        }

        d.draw_text("raylib-rs port by varomix", 20, SCREEN_HEIGHT - 20, 10, Color::GRAY);
    }
}
