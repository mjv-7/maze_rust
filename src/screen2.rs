use crate::modules::collision::check_collision;
use crate::modules::grid::draw_grid;
use crate::modules::player::Player;
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
    let door_img = StillImage::new(
        "assets/door.png",
        50.0,  // width
        180.0, // height
        150.0, // x position
        365.0, // y position
        true,  // Enable stretching
        1.0,   // Zoom level (1.0 = 100%)
    )
    .await;
    let mut keys = StillImage::new(
        "assets/keys.png",
        20.0,  // width
        20.0,  // height
        900.0, // x position
        400.0, // y position
        true,  // Enable stretching
        1.0,   // Zoom level (1.0 = 100%)
    )
    .await;
    let mut end = StillImage::new(
        "assets/end.png",
        100.0,  // width
        20.0,   // height
        890.0,  // x position
        1040.0, // y position
        true,   // Enable stretching
        1.0,    // Zoom level (1.0 = 100%)
    )
    .await;
    let mut door_exists = true;
    let mut key = false;
    let mut player = Player::new("assets/mario.png".to_string(), 350.0, 50.0, 50.0, 50.0, 50.0).await;
    // let player_keys = player::Player::new("assets/mario_keys.png".to_string(), 200.0, 50.0, 50.0, 50.0, 50.0).await;
    loop {
        clear_background(WHITE);
        player.key_press();
        if player.collision(&keys) {
            player.set_texture("assets/mario_keys.png").await;
            key = true;
            keys.clear();
        }
        if key && door_exists && player.collision(&door_img) {
            door_exists = false;
        }

        if door_exists {
            door_img.draw();
        }

        // Save old position in case of collision
        if player.collision_x(&img) || player.collision_x(&door_img) {
            player.back_x();
        }
        if player.collision_y(&img) || player.collision_y(&door_img) {
            player.back_y();
        }

        draw_text("Screen 2", 20.0, 40.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            return "screen1".to_string();
        }
        img.draw();
        keys.draw();
        player.draw();
        end.draw();
        draw_grid(50.0, BLACK);
        next_frame().await;
    }
}
