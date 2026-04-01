/*
By: Mujibullah
Date: 2026-02-11
Program Details: A simple maze game with keys to collect that can help you to open a door, and there is a wall that moves automatically.
Screen 1: Intro screen, goes to screen 2
*/

use macroquad::prelude::*;
use crate::modules::text_button::TextButton;
use crate::modules::grid::{draw_grid};
use crate::modules::label::Label;
use crate::modules::scale::use_virtual_resolution;


pub async fn run() -> String {
    use_virtual_resolution(1440.0, 1080.0);
    let mut lbl_out = Label::new("Beautiful Maze Game", 550.0, 50.0, 30);
    lbl_out.with_colors(WHITE, Some(BLACK));
     let btn_text = TextButton::new(
        750.0,
        700.0,
        200.0,
        60.0,
        "Click To Play!",
        BLACK,
        GREEN,
        30
     );
     let btn_exit = TextButton::new(
        350.0,
        700.0,
        200.0,
        60.0,
        "Exit",
        BLACK,
        GREEN,
        30
     );
    loop {
    clear_background(BLUE);
    
    draw_text("By Mujibullah", 20.0, 40.0, 30.0, WHITE);
    lbl_out.draw();
    
    if btn_text.click() || is_key_down(KeyCode::Backspace) {
        return "screen2".to_string();
    }
    if is_key_down(KeyCode::Enter) {
        return "screen3".to_string();
    }
    
    if btn_exit.click() {
        std::process::exit(0);
    }   
        
    draw_grid(50.0, BLACK);
    next_frame().await;
}
}