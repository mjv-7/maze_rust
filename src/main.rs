/*
By: Mujibullah
Date: 2026-02-11
Program Details: A simple maze game with keys to collect that can help you to open a door, and there is a wall that moves automatically.
*/

mod modules;
mod screen1;
mod screen2;
use crate::modules::grid::draw_grid;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;


/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "rust_gui_review".to_string(),
        window_width: 1440,
        window_height: 1080,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    
    
    let img = StillImage::new(
    "assets/download.png",
    1440.0,  // width
    1080.0,  // height
    0.0,  // x position
    0.0,   // y position
    true,   // Enable stretching
    1.0     // Zoom level (1.0 = 100%)
).await;
    let mut current_screen = "screen1".to_string();
    let mut last_switch = get_time() - 0.02;
    loop {
        if get_time() - last_switch > 0.01 {
            current_screen = match current_screen.as_str() {
                "screen1" => screen1::run().await,
                "screen2" => screen2::run().await,
                _ => break,
            };
            last_switch = get_time();
        }
        next_frame().await;
    }
    
    loop {
        clear_background(RED);

        img.draw();
        draw_grid(50.0, BROWN);
        next_frame().await;
    }
}
