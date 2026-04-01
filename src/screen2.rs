
/*
By: Mujibullah
Date: 2026-02-11
Program Details: A simple maze game with keys to collect that can help you to open a door, and there is a wall that moves automatically.
Screen 2: Maze game, has mostly everything for the game. At the end goes to screen 3.
*/
use crate::modules::collision::check_collision;
use crate::modules::grid::draw_grid;
use crate::modules::scale::use_virtual_resolution;
use crate::modules::player::Player;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;

pub async fn run() -> String {
    let mut wall = StillImage::new(
        "assets/blackscreen.png",
        180.0, // width
        15.0,  // height
        550.0, // x position
        900.0, // y position
        true,  // Enable stretching
        1.0,   // Zoom level (1.0 = 100%)
    )
    .await;
    let img_maze = StillImage::new(
        "assets/maze.png",
        1440.0, // width
        1080.0, // height
        0.0,    // x position
        0.0,    // y position
        true,   // Enable stretching
        1.0,    // Zoom level (1.0 = 100%)
    )
    .await;
    let mut img_door = StillImage::new(
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
    let end = StillImage::new(
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


    let mut wall_move_amount =1.0;
    loop {
         // Set the virtual resolution to 1024x768
        use_virtual_resolution(1440.0, 1080.0);
        clear_background(WHITE);
        img_maze.draw();
        keys.draw();
        player.draw();
        end.draw();
        wall.draw();
        if door_exists {
            img_door.draw();
        }
        
        draw_grid(50.0, BLACK);
        player.key_press();
        if player.collision(&keys) {
            player.set_texture("assets/mario_keys.png").await;
            key = true;
            keys.clear();
            println!("key achieved");
        }
        // println!("key: {}", key);
        //println!("door_exists: {}", door_exists);
        // println!("player.collision(&door_img): {}", player.collision(&door_img));
        wall.set_x(wall.get_x() + wall_move_amount);
        // Save old position in case of collision
        if player.collision_x(&img_maze) {
            player.back_x();
        }
        if player.collision_y(&img_maze) {
            player.back_y();
        }
        if player.collision(&wall) {
            player.back_x();
            player.back_y();
        }
        if player.collision(&end) {
            return "screen3".to_string();
        }
        if door_exists {
            if player.collision(&img_door) {
                if key {
                    door_exists = false;
                    println!("door destroyed");
                    img_door.clear();
                    player.set_texture("assets/mario.png").await;
                } else {
                    player.back_x();
                    player.back_y();
                }
            }
        }
        if check_collision(&img_maze, &wall, 1){
wall_move_amount = -wall_move_amount;
        }
        draw_text("Screen 2", 20.0, 40.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            return "screen1".to_string();
        }
        next_frame().await;
    }
}
