// raylib-rs port by varomix

extern crate raylib;

use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib-rs [shapes] example - following eyes")
        .msaa_4x()
        .build();

    let mut sclera_left_position = Vector2::new((SCREEN_WIDTH / 2 - 100) as f32, (SCREEN_HEIGHT / 2) as f32);
    let mut sclera_right_position = Vector2::new((SCREEN_WIDTH / 2 + 100) as f32, (SCREEN_HEIGHT / 2) as f32);
    let sclera_radius = 80.;

    let mut iris_left_position = Vector2::new((SCREEN_WIDTH / 2 - 100) as f32, (SCREEN_HEIGHT / 2) as f32);
    let mut iris_right_position = Vector2::new((SCREEN_WIDTH / 2 + 100) as f32, (SCREEN_HEIGHT / 2) as f32);
    let iris_radius = 24.;


    let mut angle = 0.;
    let (mut dx, mut dy, mut dxx, mut dyy) = (0., 0., 0., 0., );

    rl.set_target_fps(60);
    while !rl.window_should_close() {

        // Update
        iris_left_position = rl.get_mouse_position();
        iris_right_position = rl.get_mouse_position();

        // Check not inside the left eye sclera
        if !check_collision_point_circle(iris_left_position, sclera_left_position, sclera_radius - 20.) {
            dx = iris_left_position.x - sclera_left_position.x;
            dy = iris_left_position.y - sclera_left_position.y;

            angle = dy.atan2(dx);

            dxx = (sclera_radius - iris_radius) * angle.cos();
            dyy = (sclera_radius - iris_radius) * angle.sin();

            iris_left_position.x = sclera_left_position.x + dxx;
            iris_left_position.y = sclera_left_position.y + dyy;
        }

        // Check not inside the right eye sclera
        if !check_collision_point_circle(iris_right_position, sclera_right_position, sclera_radius - 20.) {
            dx = iris_right_position.x - sclera_right_position.x;
            dy = iris_right_position.y - sclera_right_position.y;

            angle = dy.atan2(dx);

            dxx = (sclera_radius - iris_radius) * angle.cos();
            dyy = (sclera_radius - iris_radius) * angle.sin();

            iris_right_position.x = sclera_right_position.x + dxx;
            iris_right_position.y = sclera_right_position.y + dyy;
        }


        // Draw //
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_circle_v(sclera_left_position, sclera_radius, Color::LIGHTGRAY);
        d.draw_circle_v(iris_left_position, iris_radius, Color::BROWN);
        d.draw_circle_v(iris_left_position, 10., Color::BLACK);

        d.draw_circle_v(sclera_right_position, sclera_radius, Color::LIGHTGRAY);
        d.draw_circle_v(iris_right_position, iris_radius, Color::DARKGREEN);
        d.draw_circle_v(iris_right_position, 10., Color::BLACK);

        d.draw_fps(10, 10);
        d.draw_text("raylib-rs port by varomix", 20, SCREEN_HEIGHT - 20, 10, Color::GRAY);
    }
}
