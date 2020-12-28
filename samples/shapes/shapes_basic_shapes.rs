// raylib-rs port by varomix

extern crate raylib;

use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib-rs [shapes] example - basic shapes drawing")
        .msaa_4x()
        .build();

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        // Draw //
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_text("Some basic shapes available on raylib-rs", 20, 20, 20, Color::DARKGRAY);

        d.draw_circle(SCREEN_WIDTH / 4, 120, 35., Color::DARKBLUE);
        d.draw_rectangle(SCREEN_WIDTH / 4 * 2 - 60, 100, 120, 60, Color::RED);
        d.draw_rectangle_lines(SCREEN_WIDTH / 4 * 2 - 40, 320, 80, 60, Color::ORANGE);
        d.draw_rectangle_gradient_h(SCREEN_WIDTH / 4 * 2 - 90, 170, 180, 130, Color::MAROON, Color::GOLD);

        d.draw_triangle(Vector2::new((SCREEN_WIDTH / 4 * 3) as f32, 80.),
                        Vector2::new((SCREEN_WIDTH / 4 * 3 - 60) as f32, 150.),
                        Vector2::new((SCREEN_WIDTH / 4 * 3 + 60) as f32, 150.),
                        Color::VIOLET,
        );

        d.draw_poly(Vector2::new((SCREEN_WIDTH / 4 * 3) as f32, 320.), 6, 80., 0., Color::BROWN);

        d.draw_circle_gradient(SCREEN_WIDTH / 4, 220, 60., Color::GREEN, Color::SKYBLUE);

        d.draw_line(18, 42, SCREEN_WIDTH - 18, 42, Color::BLACK);
        d.draw_circle_lines(SCREEN_WIDTH / 4, 340, 80., Color::DARKBLUE);
        d.draw_triangle_lines(Vector2::new((SCREEN_WIDTH / 4 * 3) as f32, 160.),
                              Vector2::new((SCREEN_WIDTH / 4 * 3 - 20) as f32, 230.),
                              Vector2::new((SCREEN_WIDTH / 4 * 3 + 20) as f32, 230.), Color::DARKBLUE
        );


        d.draw_text("raylib-rs port by varomix", 20, SCREEN_HEIGHT - 20, 10, Color::GRAY);
    }
}