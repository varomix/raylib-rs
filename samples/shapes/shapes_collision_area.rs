// raylib-rs port by varomix

extern crate raylib;

use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib-rs [shapes] example - collision area")
        .build();

    // Box A: Moving box
    let mut box_a = rrect(10, SCREEN_HEIGHT / 2 - 50, 200, 100);
    let mut box_a_speed_x = 4.;

    // Box B: Mouse moved box
    let mut box_b = rrect(SCREEN_WIDTH / 2 - 30, SCREEN_HEIGHT / 2 - 30, 60, 60);
    // Collision rectangle
    let mut box_collision = rrect(0, 0, 0, 0);

    let screen_upper_limit = 40.;     // Top menu limits

    let mut pause = false;          // Movement pause
    let mut collision = false;      // Collision detection

    rl.set_target_fps(60);
    while !rl.window_should_close() {

        // Update //
        // Move box if not paused
        if !pause {
            box_a.x += box_a_speed_x;
        }

        // Bounce box on x screen limits
        if box_a.x + box_a.width >= SCREEN_WIDTH as f32 || box_a.x <= 0. {
            box_a_speed_x *= -1.;
        }

        // Update player controlled box (box_b)
        box_b.x = rl.get_mouse_x() as f32 - box_b.width / 2.;
        box_b.y = rl.get_mouse_y() as f32 - box_b.height / 2.;

        // Make sure Box B does not go out of the move area limits
        if box_b.x + box_b.width >= SCREEN_WIDTH as f32 {
            box_b.x = SCREEN_WIDTH as f32 - box_b.width;
        } else if box_b.x <= 0. {
            box_b.x = 0.;
        }

        if box_b.y + box_b.height >= SCREEN_HEIGHT as f32 {
            box_b.y = SCREEN_HEIGHT as f32 - box_b.height;
        } else if box_b.y <= screen_upper_limit {
            box_b.y = screen_upper_limit;
        }

        // Check boxes collision
        collision = box_a.check_collision_recs(&box_b);

        // Get collision rectangle (only on collision)
        if collision {
            box_collision = box_a.get_collision_rec(&box_b).unwrap();
        }

        // Pause Box A movement
        if rl.is_key_pressed(KEY_SPACE) {
            pause = !pause;
        }

        // Draw //
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_rectangle(
            0,
            0,
            SCREEN_WIDTH,
            screen_upper_limit as i32,
            if collision { Color::RED } else { Color::BLACK }
        );

        d.draw_rectangle_rec(box_a, Color::GOLD);
        d.draw_rectangle_rec(box_b, Color::BLUE);

        if collision {
            // Draw collision area
            d.draw_rectangle_rec(box_collision, Color::LIME);

            // Draw Collision message
            d.draw_text(
                "COLLISION!",
                SCREEN_WIDTH / 2 - measure_text("COLLISION!", 20) / 2,
                (screen_upper_limit / 2. - 10.) as i32,
                20,
                Color::BLACK
            );

            // Draw collision area message
            d.draw_text(
                &*format!("Collision Area {}", box_collision.width * box_collision.height),
                SCREEN_WIDTH / 2 - 100,
                (screen_upper_limit + 10.) as i32,
                20,
                Color::BLACK
            );

        }
        d.draw_fps(10, 10);


        d.draw_text("raylib-rs port by varomix", 20, SCREEN_HEIGHT - 20, 10, Color::GRAY);
    }
}
