// raylib-rs port by varomix

extern crate raylib;

use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 450;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib-rs [shapes] example - draw circle sector")
        .msaa_4x()
        .build();


    let center = Vector2::new(((SCREEN_WIDTH - 300) / 2) as f32, (SCREEN_HEIGHT / 2) as f32);
    let mut outer_radius = 180f32;
    let mut start_angle = 0f32;
    let mut end_angle = 180f32;
    let mut segments = 0;

    rl.set_target_fps(60);
    while !rl.window_should_close() {
        // Draw //
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_line(500, 0, 500, SCREEN_HEIGHT, Color::fade(&Color::LIGHTGRAY, 0.6));
        d.draw_rectangle(500, 0, SCREEN_WIDTH - 500, SCREEN_HEIGHT, Color::fade(&Color::LIGHTGRAY, 0.3));

        d.draw_circle_sector(center, outer_radius, start_angle as i32, end_angle as i32, segments, Color::fade(&Color::MAROON, 0.3));
        d.draw_circle_sector_lines(center, outer_radius, start_angle as i32, end_angle as i32, segments, Color::fade(&Color::MAROON, 0.6));

        // Draw GUI controls
        start_angle = d.gui_slider_bar(rrect(600., 40., 120., 20.),
                                       Some(rstr!("StartAngle")),
                                       Some(&rstr!("{}", start_angle).unwrap()),
                                       start_angle, 0., 720.,
        );

        end_angle = d.gui_slider_bar(rrect(600., 70., 120., 20.),
                                     Some(rstr!("EndAngle")),
                                     Some(&rstr!("{}", end_angle).unwrap()),
                                     end_angle, 0., 720.,
        );

        outer_radius = d.gui_slider_bar(rrect(600., 140., 120., 20.),
                                        Some(rstr!("OuterRadius")),
                                        Some(&rstr!("{}", outer_radius).unwrap()),
                                        outer_radius, 0., 200.,
        );

        segments = d.gui_slider_bar(rrect(600., 170., 120., 20.),
                                    Some(rstr!("Segments")),
                                    Some(&rstr!("{}", segments).unwrap()),
                                    segments as f32, 0., 100.,
        ) as i32;


        d.draw_text(
            &*format!("MODE: {}", if segments >= 4 {"MANUAL"} else {"AUTO"}),
            600,
            200,
            10,
            if segments >= 4 {Color::MAROON} else {Color::DARKGRAY}
        );

        d.draw_fps(10, 10);

        d.draw_text("raylib-rs port by varomix", 20, SCREEN_HEIGHT - 20, 10, Color::GRAY);
    }
}
