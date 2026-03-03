use crate::modules::grid::draw_grid;
use crate::modules::player::{self, Player};
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

fn draw_grid_standard(grid_size: f32, color: Color) {
    let screen_width = screen_width();
    let screen_height = screen_height();

    // Draw vertical lines and labels
    for x in (0..screen_width as i32).step_by(grid_size as usize) {
        draw_line(x as f32, 0.0, x as f32, screen_height, 1.0, color);
        draw_text(&format!("{}", x), x as f32 + 2.0, 12.0, 16.0, color);
    }

    // Draw horizontal lines and labels
    for y in (0..screen_height as i32).step_by(grid_size as usize) {
        draw_line(0.0, y as f32, screen_width, y as f32, 1.0, color);
        draw_text(&format!("{}", y), 2.0, y as f32 + 12.0, 16.0, color);
    }
}

pub async fn run() -> String {
    let img = StillImage::new(
        "assets/maze.png",
        1440.0, // width
        1080.0, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Zoom level (1.0 = 100%)
    )
    .await;
    let mut player = player::Player::new("assets/mario.png".to_string(), 200.0, 50.0, 50.0, 50.0, 50.0).await;

    loop {
        clear_background(WHITE);

        player.key_press();
        println!("{:?}", player.movement());
        // Save old position in case of collision
        if player.collision_x(&img) {
            player.back();
        }
        if player.collision_y(&img) {
            player.back();
        }
        draw_text("Screen 2", 20.0, 40.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            return "screen1".to_string();
        }
        img.draw();
        player.draw();
        draw_grid(50.0, BLACK);
        next_frame().await;
    }
}
